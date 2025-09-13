macro_rules! is_same {
    ($left:ty, $($right:ty),+ $(,)*) => {
        $( ::std::any::TypeId::of::<$left>() == ::std::any::TypeId::of::<$right>() && )+ true
    };
}

macro_rules! is_type_of {
    ($left:expr, $right:ty $(,)?) => {{
        let boxed_left: Box<dyn ::std::any::Any> = Box::new($left);

        (&*boxed_left).type_id() == ::std::any::TypeId::of::<$right>()
    }};
    ($left:expr, $right:expr $(,)?) => {{
        let boxed_left: Box<dyn ::std::any::Any> = Box::new($left);
        let boxed_right: Box<dyn ::std::any::Any> = Box::new($right);

        (&*boxed_left).type_id() == (&*boxed_right).type_id()
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_is_same_tp_two_args() {
        assert!(is_same!(i8, i8));
    }

    fn test_is_same_tp_three_args() {
        assert!(is_same!(i8, i8, i8));
    }

    fn test_is_same_tp_four_args() {
        assert!(is_same!(i8, i8, i8, i8));
    }

    fn test_is_same_tn_two_args() {
        assert!(!is_same!(i8, i16));
    }

    fn test_is_same_tn_three_args1() {
        assert!(!is_same!(i8, i8, i16));
    }

    fn test_is_same_tn_three_args2() {
        assert!(!is_same!(i8, i16, i8));
    }
}
