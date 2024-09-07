mod either {
    // Left -> Error
    // Right -> Correct value
    pub enum Either<L, R> {
        Left(L),
        Right(R),
    }

    impl<L, R> Either<L, R> {
        pub fn left(value: L) -> Self {
            Either::Left(value)
        }

        pub fn right(value: R) -> Self {
            Either::Right(value)
        }

        pub fn is_left(&self) -> bool {
            match self {
                Either::Left(_) => true,
                Either::Right(_) => false,
            }
        }

        pub fn is_right(&self) -> bool {
            match self {
                Either::Left(_) => false,
                Either::Right(_) => true,
            }
        }

        // This function takes ownership of self
        pub fn unwrap(self) -> R {
            match self {
                Either::Left(_) => {
                    panic!("Called Either::unwrap() on Left value!")
                }
                Either::Right(value) => value,
            }
        }
    }

    #[macro_export]
    macro_rules! try_either {
        ($expr:expr) => {
            match $expr {
                Either::Right(value) => value,
                Either::Left(error) => return Either::left(error),
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::either::either::Either;
    use crate::try_either;

    #[test]
    fn should_exit_on_left() {
        let func = || -> Either<i32, String> {
            try_either!(Either::left(15));
            Either::right(String::from("Hello"))
        };

        let result = func();

        assert_eq!(result.is_left(), true)
    }

    #[test]
    fn should_continue_on_right() {
        let func = || -> Either<String, i32> {
            let value = try_either!(Either::right(10));
            Either::right(value + 10)
        };

        let result = func();

        assert_eq!(result.is_right(), true);
        assert_eq!(result.unwrap(), 20);
    }
}
