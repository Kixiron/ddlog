use crate::providers::semantic_tokens::tokens::{ModifierSet, SUPPORTED_TYPES};
use lspower::lsp::{Range, SemanticToken, SemanticTokenType, SemanticTokens, SemanticTokensEdit};

/// Tokens are encoded relative to each other.
///
/// This is a direct port of <https://github.com/microsoft/vscode-languageserver-node/blob/f425af9de46a0187adb78ec8a46b9b2ce80c5412/server/src/sematicTokens.proposed.ts#L45>
#[derive(Debug)]
pub(crate) struct SemanticTokensBuilder {
    // TODO: Make this non-optional
    id: Option<String>,
    prev_line: u32,
    prev_char: u32,
    data: Vec<SemanticToken>,
}

impl SemanticTokensBuilder {
    // pub(crate) fn new(id: Option<String>) -> Self {
    //     Self::with_capacity(id, 0)
    // }

    pub(crate) fn with_capacity(id: Option<String>, capacity: usize) -> Self {
        Self {
            id,
            prev_line: 0,
            prev_char: 0,
            data: Vec::with_capacity(capacity),
        }
    }

    /// Push a new token onto the builder
    pub(crate) fn push(&mut self, range: Range, token: SemanticTokenType, modifiers: ModifierSet) {
        let mut push_line = range.start.line as u32;
        let mut push_char = range.start.character as u32;

        if !self.data.is_empty() {
            push_line -= self.prev_line;

            if push_line == 0 {
                push_char -= self.prev_char;
            }
        }

        // A token cannot be multiline
        let token_len = range.end.character - range.start.character;

        let token = SemanticToken {
            delta_line: push_line,
            delta_start: push_char,
            length: token_len as u32,
            token_type: type_index(token),
            token_modifiers_bitset: modifiers.0,
        };

        self.data.push(token);

        self.prev_line = range.start.line as u32;
        self.prev_char = range.start.character as u32;
    }

    pub(crate) fn build(self) -> SemanticTokens {
        SemanticTokens {
            result_id: self.id,
            data: self.data,
        }
    }
}

#[allow(dead_code)]
pub(crate) fn diff_tokens(old: &[SemanticToken], new: &[SemanticToken]) -> Vec<SemanticTokensEdit> {
    let offset = new
        .iter()
        .zip(old.iter())
        .take_while(|&(n, p)| n == p)
        .count();

    let (_, old) = old.split_at(offset);
    let (_, new) = new.split_at(offset);

    let offset_from_end = new
        .iter()
        .rev()
        .zip(old.iter().rev())
        .take_while(|&(n, p)| n == p)
        .count();

    let (old, _) = old.split_at(old.len() - offset_from_end);
    let (new, _) = new.split_at(new.len() - offset_from_end);

    if old.is_empty() && new.is_empty() {
        Vec::new()
    } else {
        // The lsp data field is actually a byte-diff but we
        // travel in tokens so `start` and `delete_count` are in multiples of the
        // serialized size of `SemanticToken`.
        vec![SemanticTokensEdit {
            start: 5 * offset as u32,
            delete_count: 5 * old.len() as u32,
            data: Some(new.to_vec()),
        }]
    }
}

fn type_index(ty: SemanticTokenType) -> u32 {
    SUPPORTED_TYPES.iter().position(|it| *it == ty).unwrap() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    fn from(t: (u32, u32, u32, u32, u32)) -> SemanticToken {
        SemanticToken {
            delta_line: t.0,
            delta_start: t.1,
            length: t.2,
            token_type: t.3,
            token_modifiers_bitset: t.4,
        }
    }

    #[test]
    fn test_diff_insert_at_end() {
        let before = [from((1, 2, 3, 4, 5)), from((6, 7, 8, 9, 10))];
        let after = [
            from((1, 2, 3, 4, 5)),
            from((6, 7, 8, 9, 10)),
            from((11, 12, 13, 14, 15)),
        ];

        let edits = diff_tokens(&before, &after);
        assert_eq!(
            edits[0],
            SemanticTokensEdit {
                start: 10,
                delete_count: 0,
                data: Some(vec![from((11, 12, 13, 14, 15))])
            }
        );
    }

    #[test]
    fn test_diff_insert_at_beginning() {
        let before = [from((1, 2, 3, 4, 5)), from((6, 7, 8, 9, 10))];
        let after = [
            from((11, 12, 13, 14, 15)),
            from((1, 2, 3, 4, 5)),
            from((6, 7, 8, 9, 10)),
        ];

        let edits = diff_tokens(&before, &after);
        assert_eq!(
            edits[0],
            SemanticTokensEdit {
                start: 0,
                delete_count: 0,
                data: Some(vec![from((11, 12, 13, 14, 15))])
            }
        );
    }

    #[test]
    fn test_diff_insert_in_middle() {
        let before = [from((1, 2, 3, 4, 5)), from((6, 7, 8, 9, 10))];
        let after = [
            from((1, 2, 3, 4, 5)),
            from((10, 20, 30, 40, 50)),
            from((60, 70, 80, 90, 100)),
            from((6, 7, 8, 9, 10)),
        ];

        let edits = diff_tokens(&before, &after);
        assert_eq!(
            edits[0],
            SemanticTokensEdit {
                start: 5,
                delete_count: 0,
                data: Some(vec![
                    from((10, 20, 30, 40, 50)),
                    from((60, 70, 80, 90, 100))
                ])
            }
        );
    }

    #[test]
    fn test_diff_remove_from_end() {
        let before = [
            from((1, 2, 3, 4, 5)),
            from((6, 7, 8, 9, 10)),
            from((11, 12, 13, 14, 15)),
        ];
        let after = [from((1, 2, 3, 4, 5)), from((6, 7, 8, 9, 10))];

        let edits = diff_tokens(&before, &after);
        assert_eq!(
            edits[0],
            SemanticTokensEdit {
                start: 10,
                delete_count: 5,
                data: Some(vec![])
            }
        );
    }

    #[test]
    fn test_diff_remove_from_beginning() {
        let before = [
            from((11, 12, 13, 14, 15)),
            from((1, 2, 3, 4, 5)),
            from((6, 7, 8, 9, 10)),
        ];
        let after = [from((1, 2, 3, 4, 5)), from((6, 7, 8, 9, 10))];

        let edits = diff_tokens(&before, &after);
        assert_eq!(
            edits[0],
            SemanticTokensEdit {
                start: 0,
                delete_count: 5,
                data: Some(vec![])
            }
        );
    }

    #[test]
    fn test_diff_remove_from_middle() {
        let before = [
            from((1, 2, 3, 4, 5)),
            from((10, 20, 30, 40, 50)),
            from((60, 70, 80, 90, 100)),
            from((6, 7, 8, 9, 10)),
        ];
        let after = [from((1, 2, 3, 4, 5)), from((6, 7, 8, 9, 10))];

        let edits = diff_tokens(&before, &after);
        assert_eq!(
            edits[0],
            SemanticTokensEdit {
                start: 5,
                delete_count: 10,
                data: Some(vec![])
            }
        );
    }
}
