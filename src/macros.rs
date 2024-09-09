pub mod macros {
    use std::fmt::Display;

    #[macro_export]
    macro_rules! count_tts {
        () => { 0 };
        ($odd:tt $($a:tt $b:tt)*) => { (count_tts!($($a)*) << 1) | 1 };
        ($($a:tt $even:tt)*) => { count_tts!($($a)*) << 1 };
    }

    #[macro_export]
    macro_rules! vector {
        ($t:ty) => {
            {
                let vec: Vec<$t> = Vec::with_capacity(8);
                vec
            }
        };
    }

    #[macro_export]
    macro_rules! create_func {
        ($i:ident, $l:literal) => {
            fn $i() -> String {
                String::from($l)
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::{create_func, vector};

    #[test]
    fn should_create_vector_of_specified_type() {
        let mut result: Vec<Box<i32>> = vector!(Box<i32>);

        result.push(Box::new(7));

        assert_eq!(*result[0], 7);
    }

    #[test]
    fn should_call_created_function() {
        create_func!(func, "Hello");

        let result = func();

        assert_eq!(result, String::from("Hello"));
    }
}