use crate::{
    parser::{CompletedMarker, Parser},
    SyntaxKind::{
        FUNCTION_RETURN_TYPE, FUNCTION_TYPE, FUNCTION_TYPE_ARG, FUNCTION_TYPE_ARGS, GENERICS,
        GENERIC_ARG, GENERIC_TYPE, IDENT, PATH, TUPLE_TYPE, TUPLE_TYPE_ELEM, VAR_REF,
    },
};

impl Parser<'_, '_> {
    // TODO: Extend to full types
    pub(super) fn ty(&mut self) -> Option<CompletedMarker> {
        let _frame = self.stack_frame();

        match self.current() {
            IDENT => self.type_name(),
            T!['('] => self.tuple_type(),
            T![fn] => self.function_type(),

            // FIXME: Error
            _ => None,
        }
    }

    // test generic_types
    // - fn foo(bar: Bar<Baz>) {}
    fn type_name(&mut self) -> Option<CompletedMarker> {
        let generic = self.start();

        if self.at(IDENT) {
            let path = self.start();
            self.expect(IDENT);
            path.complete(self, PATH);

            if self.at(T![<]) {
                let generics = self.start();

                self.expect(T![<]);
                while !self.at(T![>]) {
                    let generic_arg = self.start();

                    self.ty();
                    self.eat_commas();

                    generic_arg.complete(self, GENERIC_ARG);
                }

                self.expect(T![>]);
                generics.complete(self, GENERICS);
            }
        }

        Some(generic.complete(self, GENERIC_TYPE))
    }

    // test tuple_types
    // - fn foo(bar: (Bar, Baz)) {}
    fn tuple_type(&mut self) -> Option<CompletedMarker> {
        let tuple = self.start();
        self.expect(T!['(']);

        while !self.at(T![')']) {
            let elem = self.start();

            self.ty();
            self.eat_commas();

            elem.complete(self, TUPLE_TYPE_ELEM);
        }

        self.expect(T![')']);
        Some(tuple.complete(self, TUPLE_TYPE))
    }

    // test function_types
    // - fn foo(bar: fn(Bar, Baz) -> Bing) {}
    // - fn Foo(bar: fn(Bar, Baz,) -> Bing,) {}
    // - fn foo(bar: fn(Bar)) {}
    // - fn Foo(bar: fn(Bar,)) -> fn(Bong, Bang) {}
    fn function_type(&mut self) -> Option<CompletedMarker> {
        let function = self.start();

        self.expect(T![fn]);

        {
            let args = self.start();
            self.expect(T!['(']);

            while !self.at(T![')']) {
                let arg = self.start();

                self.ty();
                self.eat_commas();

                arg.complete(self, FUNCTION_TYPE_ARG);
            }

            self.expect(T![')']);
            args.complete(self, FUNCTION_TYPE_ARGS);
        }

        {
            let ret = self.start();

            // FIXME: Error recovery for missing `->`
            if self.try_expect(T![->]) {
                self.ty();
            }

            ret.complete(self, FUNCTION_RETURN_TYPE);
        }

        Some(function.complete(self, FUNCTION_TYPE))
    }

    // TODO: Extend to full patterns
    pub(super) fn pattern(&mut self) -> Option<CompletedMarker> {
        let _frame = self.stack_frame();
        let pattern = self.start();

        self.expect(IDENT);

        Some(pattern.complete(self, VAR_REF))
    }
}
