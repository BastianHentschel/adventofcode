#[macro_export]
macro_rules! runners {
    ($year:ident, $($day:ident),*) => {
        $(
            $year::$day::runner(format!("data/{}/{}.txt", stringify!($year), stringify!($day)));
        )*
    };

}