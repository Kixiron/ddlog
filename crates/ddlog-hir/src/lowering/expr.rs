use crate::{
    hir::{BinExpr, BinOp, Expr, FunctionCall, Literal, LiteralTypeHint, Match, MatchArm, Pattern},
    lowering::HirBuilder,
};
use ddlog_diagnostics::IStr;
use ddlog_syntax::ast::{
    prefixed::nodes::{AstBinExpr, AstExpr, AstFunctionCall, AstIfExpr, AstLiteral},
    AstNode, AstToken,
};

impl<'a> HirBuilder<'a> {
    pub(super) fn expr<E>(&self, expr: E) -> Option<Expr>
    where
        E: AsRef<AstExpr>,
    {
        self.expr_inner(expr.as_ref())
    }

    fn expr_inner(&self, expr: &AstExpr) -> Option<Expr> {
        match expr {
            AstExpr::VarRef(var) => var.interned().map(Expr::VarRef),
            AstExpr::RetExpr(ret) => Some(Expr::Return(Box::new(self.expr(ret.expr()?)?))),
            AstExpr::Literal(literal) => self.literal(literal).map(Expr::Literal),
            AstExpr::BinExpr(bin_expr) => self.bin_expr(bin_expr).map(Expr::BinOp),
            AstExpr::IfExpr(if_expr) => self.if_expr(if_expr).map(Expr::Match),
            AstExpr::FunctionCall(call) => self.function_call(call).map(Expr::FunctionCall),

            expr => todo!("{}", expr.syntax().debug(self.interner, true)),
        }
    }

    // We desugar if-else blocks into matches from this
    //
    // ```
    // if x {
    //     a
    // } else {
    //     b
    // }
    // ```
    //
    // Into this
    //
    // ```
    // match x {
    //     true => a,
    //     false => b,
    // }
    // ```
    //
    // However, if-else statements that include `else if` clauses are
    // desugared differently from this
    //
    // ```
    // if x {
    //     a
    // } else if y {
    //     b
    // } else {
    //     c
    // }
    // ```
    //
    // Into this
    //
    // ```
    // match () {
    //     () if x => a,
    //     () if y => b,
    //     () => c,
    // }
    // ```
    // TODO: Should we desugar them as this instead?
    // ```
    // match x {
    //     true => a,
    //     false => match y {
    //         true => b,
    //         false => c,
    //     },
    // }
    // ```
    //
    // TODO: Handle else-if clauses
    fn if_expr(&self, if_expr: &AstIfExpr) -> Option<Match> {
        let mut ifs = if_expr.if_blocks();
        let if_block = ifs.next()?;
        if ifs.count() != 0 {
            todo!("handle if-else clauses");
        }

        let scrutinee = Box::new(self.expr(if_block.cond()?)?);
        let mut arms = Vec::with_capacity(2);

        // Build the true arm from the if block
        let true_arm = self.block(if_block.block()?)?;
        arms.push(MatchArm::new(
            Pattern::Literal(Literal::Bool(true)),
            None,
            Box::new(Expr::Block(true_arm)),
        ));

        if let Some(else_block) = if_expr.else_block() {
            let false_arm = self.block(else_block.block()?)?;

            // Build the false arm from the else block
            arms.push(MatchArm::new(
                Pattern::Literal(Literal::Bool(false)),
                None,
                Box::new(Expr::Block(false_arm)),
            ));

        // If no else block is provided, use a unit literal as the false block.
        // This is correct since if there's only one block to the if, the statement
        // must have a unit type overall
        } else {
            arms.push(MatchArm::new(
                Pattern::Literal(Literal::Bool(false)),
                None,
                Box::new(Expr::Literal(Literal::Unit)),
            ));
        }

        Some(Match::new(scrutinee, arms))
    }

    fn function_call(&self, call: &AstFunctionCall) -> Option<FunctionCall> {
        let func = self.expr(call.func()?)?;
        let args = call
            .args()
            .map(|arg| self.expr(arg.arg()?))
            .collect::<Option<Vec<_>>>()?;

        Some(FunctionCall::new(Box::new(func), args))
    }

    fn literal(&self, literal: &AstLiteral) -> Option<Literal> {
        Some(match literal {
            AstLiteral::Bool(bool) => {
                if bool.is_true() {
                    Literal::Bool(true)
                } else if bool.is_false() {
                    Literal::Bool(false)
                } else {
                    Literal::Error(Some(LiteralTypeHint::Bool))
                }
            }

            AstLiteral::Char(_) => todo!(),

            AstLiteral::Number(number) => {
                let literal = number.number_literal()?;
                let number = literal.text(self.interner);

                // TODO: Should we pick a different repr than a u128?
                Literal::Number(number.parse::<u128>().unwrap())
            }

            AstLiteral::String(string) => {
                Literal::String(IStr::new(string.string_literal()?.syntax().text_key()))
            }
        })
    }

    fn bin_expr(&self, bin_expr: &AstBinExpr) -> Option<BinExpr> {
        let lhs = Box::new(self.expr(bin_expr.lhs()?)?);
        let rhs = Box::new(self.expr(bin_expr.rhs()?)?);

        // TODO: Allow matching on binop kinds
        let op = bin_expr.op()?;
        let op = if op.is_or() {
            BinOp::Or
        } else if op.is_and() {
            BinOp::And
        } else if op.is_plus() {
            BinOp::Add
        } else if op.is_minus() {
            BinOp::Sub
        } else if op.is_star() {
            BinOp::Mul
        } else if op.is_slash() {
            BinOp::Div
        } else if op.is_eqeq() {
            BinOp::Eq
        } else if op.is_neq() {
            BinOp::Neq
        } else if op.is_l_angle() {
            BinOp::Less
        } else if op.is_l_angle_eq() {
            BinOp::LessEq
        } else if op.is_r_angle() {
            BinOp::Greater
        } else if op.is_r_angle_eq() {
            BinOp::GreaterEq
        } else {
            return None;
        };

        Some(BinExpr::new(lhs, rhs, op))
    }
}
