#[macro_export]
macro_rules! single_map {
    ($k:expr, $v:expr) => {
        HashMap::from([($k, $v)])
    };
}
