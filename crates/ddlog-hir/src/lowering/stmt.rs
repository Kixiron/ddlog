use crate::{
    hir::{Stmt, VarDecl},
    lowering::HirBuilder,
};
use ddlog_syntax::ast::prefixed::nodes::{AstBlock, AstStmt, AstVarDecl};

impl<'a> HirBuilder<'a> {
    pub(super) fn block<B>(&self, block: B) -> Option<Vec<Stmt>>
    where
        B: AsRef<AstBlock>,
    {
        self.block_inner(block.as_ref())
    }

    fn block_inner(&self, block: &AstBlock) -> Option<Vec<Stmt>> {
        block
            .statements()
            .map(|stmt| self.stmt(stmt))
            .collect::<Option<Vec<_>>>()
    }

    pub(super) fn stmt<S>(&self, stmt: S) -> Option<Stmt>
    where
        S: AsRef<AstStmt>,
    {
        self.stmt_inner(stmt.as_ref())
    }

    fn stmt_inner(&self, stmt: &AstStmt) -> Option<Stmt> {
        match stmt {
            AstStmt::VarDecl(decl) => self.var_decl(decl).map(Stmt::VarDecl),

            // TODO: Semicolon termination or desugaring trailing expressions to a return expression
            AstStmt::ExprStmt(expr) => self.expr(expr.expr()?).map(Stmt::Expr),
        }
    }

    fn var_decl(&self, decl: &AstVarDecl) -> Option<VarDecl> {
        let binding = self.pattern(&*decl.binding()?)?;
        let value = self.expr(decl.value()?)?;
        let ty = decl.ty().map(|ty| self.ty(ty).unwrap());

        Some(VarDecl::new(binding, ty, value))
    }
}
