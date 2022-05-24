// mod extra_commas;
// mod modifiers;
mod struct_field_count;

use crate::{
    visitor::{self, RuleCtx},
    AstVisitor, SyntaxNode,
};
use ddlog_utils::bvec;
use struct_field_count::StructFieldCount;
// use extra_commas::ExtraCommas;
// use modifiers::ModifierValidator;

pub fn run_validators(node: &SyntaxNode, ctx: &mut RuleCtx) {
    let validators = validators();

    for mut validator in validators {
        visitor::apply_visitor(node, &mut validator, ctx);
    }
}

fn validators() -> Vec<Box<dyn AstVisitor>> {
    // vec![Box::new(ModifierValidator), Box::new(ExtraCommas)]
    bvec![StructFieldCount::default()]
}
