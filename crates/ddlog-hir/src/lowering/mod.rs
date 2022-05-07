mod expr;
mod items;
mod stmt;
mod types;

use crate::hir::{Item, Path, Pattern};
use ddlog_diagnostics::Interner;
use ddlog_syntax::ast::prefixed::nodes::{AstItem, AstPath, AstPattern};

pub struct HirBuilder<'a> {
    interner: &'a Interner,
}

impl<'a> HirBuilder<'a> {
    pub fn new(interner: &'a Interner) -> Self {
        Self { interner }
    }

    pub fn lower_items<I, T>(&mut self, items: I) -> Vec<Item>
    where
        I: IntoIterator<Item = T>,
        T: AsRef<AstItem>,
    {
        items
            .into_iter()
            .map(|item| self.item(item.as_ref()))
            .collect()
    }

    fn pattern(&self, binding: &AstPattern) -> Option<Pattern> {
        match binding {
            AstPattern::VarRef(var) => var.interned().map(Pattern::VarRef),
            AstPattern::StructPattern(_) | AstPattern::TuplePattern(_) => todo!(),
        }
    }

    fn path<P>(&self, path: P) -> Option<Path>
    where
        P: AsRef<AstPath>,
    {
        fn path_inner(path: &AstPath) -> Option<Path> {
            let head = path.head()?.interned();
            let tail = path.tails();

            let mut segments = Vec::with_capacity(1 + tail.len());
            segments.push(head);

            for segment in tail {
                segments.push(segment.tail()?.interned());
            }

            debug_assert!(!segments.is_empty());
            Some(Path::new(segments))
        }

        path_inner(path.as_ref())
    }
}
