#[macro_export]
#[allow(unused_macros)]
macro_rules! err_if_fail {
    ($e: expr $(,)?) => {
        match $e {
            Ok(x) => x,
            Err(e) => return Err(e.to_string()),
        }
    };
}
