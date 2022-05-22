use crate::{
    hir::{FuncDef, FuncParam, Item, Type},
    lowering::HirBuilder,
};
use ddlog_syntax::ast::prefixed::nodes::{AstFunctionDef, AstItem};

impl<'a> HirBuilder<'a> {
    pub(super) fn item(&self, item: &AstItem) -> Item {
        match item {
            AstItem::FunctionDef(func) => self.function(func).map(Item::FuncDef).unwrap(),

            AstItem::ConstDef(_)
            | AstItem::EnumDef(_)
            | AstItem::ImplBlock(_)
            | AstItem::StructDef(_)
            | AstItem::TypeAlias(_)
            | AstItem::UseDef(_)
            | AstItem::RelationDef(_)
            | AstItem::ClauseDef(_) => todo!(),
        }
    }

    // TODO: We could make this more resilient, but that would require making the hir more
    //       tolerant to the loss of information. Do this later, once we've figured everything
    //       else out
    fn function(&self, func: &AstFunctionDef) -> Option<FuncDef> {
        let name = func.ident()?;
        let params = func
            .args()?
            .args()
            .map(|arg| {
                let binding = self.pattern(&*arg.binding()?)?;
                let ty = self.ty(arg.ty()?)?;

                Some(FuncParam::new(binding, ty))
            })
            .collect::<Option<Vec<_>>>()?;

        let body = self.block(func.body()?)?;

        let return_ty = if let Some(ret) = func.ret() {
            self.ty(ret.return_ty()?)?

        // Default to a unit type
        } else {
            Type::Unit
        };

        Some(FuncDef::new(name, params, body, return_ty))
    }
}
