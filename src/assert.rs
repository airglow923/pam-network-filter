#[macro_export]
#[allow(unused_macros)]
macro_rules! assert_eq_type {
    ($left:expr, $right:ty $(,)?) => {
        assert!(is_type_of!($left, $right));
    };
    ($left:expr, $right:expr $(,)?) => {
        assert!(is_type_of!($left, $right));
    };
}

#[macro_export]
#[allow(unused_macros)]
macro_rules! assert_ne_type {
    ($left:expr, $right:ty $(,)?) => {
        assert!(!is_type_of!($left, $right));
    };
    ($left:expr, $right:expr $(,)?) => {
        assert!(!is_type_of!($left, $right));
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_assert_eq_type_tp_expr_ty() {
        let expr: i8 = 1;
        assert_eq_type!(expr, i8);
    }

    #[test]
    fn test_assert_ne_type_tp_expr_ty() {
        let expr: i8 = 1;
        assert_ne_type!(expr, i16);
    }
}
