use crate::{ast::nodes::StructDef, AstVisitor, RuleCtx, SyntaxNode, SyntaxNodeExt};
use ddlog_diagnostics::{Diagnostic, Label, Span};

// We allow a maximum of (2^16)-1 struct fields in a single struct
const MAX_FIELDS: usize = u16::MAX as usize;

#[derive(Default)]
pub struct StructFieldCount;

impl AstVisitor for StructFieldCount {
    fn check_node(&mut self, node: &SyntaxNode, ctx: &mut RuleCtx) -> Option<()> {
        let strct = node.try_to::<StructDef>()?;
        let fields = strct.fields()?;
        let total_fields = fields.len();

        if total_fields > MAX_FIELDS {
            let span = Span::from_text_range(strct.signature_span(), ctx.file());
            dbg!(span);

            let note = if let Some(name) = strct.ident() {
                format!(
                    "ddlog allows a maximum of {MAX_FIELDS} struct fields, but {} has {total_fields}",
                    ctx.interner().resolve(name),
                )
            } else {
                format!(
                    "ddlog allows a maximum of {MAX_FIELDS} struct fields, but the current struct has {total_fields}",
                )
            };

            let error = Diagnostic::error()
                .with_message(format!("struct contains more than {MAX_FIELDS} fields"))
                .with_message_span(span)
                .with_label(Label::primary(span).with_message(note));

            ctx.push_diagnostic(error);
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::MAX_FIELDS;
    use crate::{validation::struct_field_count::StructFieldCount, visitor, NodeCache, RuleCtx};
    use ddlog_diagnostics::{DiagnosticConfig, FileCache, FileId, Interner, Rope};

    #[test]
    fn too_many_struct_fields() {
        let mut source = String::from("struct Foo {\n");
        for i in 0..MAX_FIELDS + 1 {
            source.push_str(&format!("    field_{i}: (),\n"));
        }
        source.push_str("}\n");

        let interner = Interner::new();
        let file = FileId::new(interner.get_or_intern_static("tests/too_many_struct_fields.ddlog"));

        let diagnostic_config = DiagnosticConfig::new().with_color(false);
        let mut file_cache = FileCache::new(interner.clone());
        file_cache.add_str(file, &source);

        let cache = NodeCache::from_interner(interner);
        let (parsed, cache) = crate::parse(file, &source, cache);

        let mut visitor = StructFieldCount::default();
        let mut ctx = RuleCtx::new(file, Rope::from_str(&source), cache.interner().clone());
        visitor::apply_visitor(parsed.syntax(), &mut visitor, &mut ctx);

        let errors = ctx.into_diagnostics();

        let (mut output, mut buffer) = (String::new(), Vec::new());
        for diagnostic in errors {
            diagnostic
                .emit_to(&diagnostic_config, &mut file_cache, &mut buffer)
                .expect("failed to emit diagnostic");

            output.push_str(&format!(
                "--\n{}",
                std::str::from_utf8(buffer.as_slice()).expect("non utf8 in error buffer"),
            ));

            buffer.clear();
        }
        output.push_str(&format!("--\n{}", &source));

        expect_test::expect_file!["./too_many_struct_files.expected"].assert_eq(&output);
    }
}
