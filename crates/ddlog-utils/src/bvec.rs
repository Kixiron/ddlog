#[macro_export]
macro_rules! bvec {
    () => { ::std::vec::Vec::new() };

    ($($item:expr),* $(,)?) => {
        ::std::vec![
            $(::std::boxed::Box::new($item),)*
        ]
    };

    ($item:expr; $count:expr) => {
        ::std::vec![::std::boxed::Box::new($item); $count]
    }
}
