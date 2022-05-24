use std::fmt::Debug;

pub trait OptionExt<T> {
    fn debug_unwrap_none(self)
    where
        T: Debug;

    fn debug_expect_none(self, message: &str);
}

impl<T> OptionExt<T> for Option<T> {
    #[inline]
    #[track_caller]
    fn debug_unwrap_none(self)
    where
        T: Debug,
    {
        #[cold]
        #[track_caller]
        #[inline(never)]
        fn debug_unwrap_panic(value: &dyn Debug) -> ! {
            panic!("called `.debug_unwrap_none()` on a `Some` value: {value:?}")
        }

        if cfg!(debug_assertions) {
            if let Some(inner) = self {
                debug_unwrap_panic(&inner);
            }
        }
    }

    #[inline]
    #[track_caller]
    fn debug_expect_none(self, message: &str) {
        #[cold]
        #[track_caller]
        #[inline(never)]
        fn debug_expect_none(value: &str) -> ! {
            panic!("called `.debug_unwrap_none()` on a `Some` value: {value:?}")
        }

        if cfg!(debug_assertions) && self.is_some() {
            debug_expect_none(message)
        }
    }
}
