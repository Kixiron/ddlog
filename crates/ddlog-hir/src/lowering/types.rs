use crate::{
    hir::{GenericType, TupleType, Type},
    lowering::HirBuilder,
};
use ddlog_syntax::ast::prefixed::nodes::{AstGenericType, AstTupleType, AstType};

impl<'a> HirBuilder<'a> {
    pub(super) fn ty<T>(&self, ty: T) -> Option<Type>
    where
        T: AsRef<AstType>,
    {
        self.ty_inner(ty.as_ref())
    }

    fn ty_inner(&self, ty: &AstType) -> Option<Type> {
        match ty {
            AstType::GenericType(generic) => self.generic_type(generic).map(Type::GenericType),
            AstType::TupleType(tuple) => self.tuple_type(tuple),
            AstType::FunctionType(_) => todo!(),
        }
    }

    fn generic_type(&self, generic: &AstGenericType) -> Option<GenericType> {
        let path = self.path(generic.path()?)?;
        let generics = generic
            .generics()
            .map(|generic| {
                generic
                    .generics()
                    .map(|generic| generic.ty().and_then(|ty| self.ty(ty)))
                    .collect::<Option<Vec<_>>>()
            })
            .unwrap_or_else(|| Some(Vec::new()))?;

        Some(GenericType::new(path, generics))
    }

    fn tuple_type(&self, tuple: &AstTupleType) -> Option<Type> {
        let elements = tuple
            .elements()
            .map(|elem| elem.ty().and_then(|ty| self.ty(ty)))
            .collect::<Option<Vec<_>>>()?;

        // If there's no tuple elements, this is the unit type
        if elements.is_empty() {
            Some(Type::Unit)

        // Otherwise this is a tuple type
        } else {
            Some(Type::TupleType(TupleType::new(elements)))
        }
    }
}
