use crate::ast::{
    nodes::{Attribute, EnumDef, EnumVariant, FunctionDef},
    AstNode, AstToken,
};
use cstree::TextRange;
use ddlog_diagnostics::Interner;

impl FunctionDef {
    /// Get the span of a function's signature
    ///
    /// If `include_keyword` is set then the `function` keyword will be
    /// included in the signature's span
    pub fn signature_span(&self, include_keyword: bool) -> TextRange {
        let keyword = self.keyword().map(|function| function.text_range());
        let name = self.name().as_ref().map(|name| name.text_range());
        let generics = self.generics().map(|generics| generics.trimmed_range());
        let args = self.args().map(|args| args.trimmed_range());
        let ret = self.ret().map(|ret| ret.trimmed_range());

        let start = if include_keyword {
            keyword.or(name)
        } else {
            name
        }
        .or(generics)
        .or(args)
        .or(ret)
        .unwrap_or_else(|| self.trimmed_range());

        let end = ret
            .or(args)
            .or(generics)
            .or(name)
            .or_else(|| if include_keyword { keyword } else { None })
            .unwrap_or_else(|| self.trimmed_range());

        start.intersect(end).unwrap_or_else(|| self.trimmed_range())
    }

    /// Returns `true` if the enum has a `#[deprecated]` attribute
    pub fn is_deprecated(&self, interner: &Interner) -> bool {
        self.attributes().any(|attr| attr.is_deprecated(interner))
    }
}

impl EnumDef {
    /// Returns `true` if the enum has a `#[deprecated]` attribute
    pub fn is_deprecated(&self, interner: &Interner) -> bool {
        self.attributes().any(|attr| attr.is_deprecated(interner))
    }
}

impl EnumVariant {
    /// Returns `true` if the variant has a `#[deprecated]` attribute
    pub fn is_deprecated(&self, interner: &Interner) -> bool {
        self.attributes().any(|attr| attr.is_deprecated(interner))
    }
}

impl Attribute {
    /// Returns `true` if any of the underlying attributes are `deprecated`
    pub fn is_deprecated(&self, interner: &Interner) -> bool {
        for pair in self.attr_pairs() {
            if let Some(name) = pair.name() {
                if name.lexical_eq("deprecated", interner) {
                    return true;
                }
            }
        }

        false
    }
}
