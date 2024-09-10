pub mod either {
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

    pub enum EitherIterator<'a, R> {
        Ref(&'a R),
        None,
    }

    impl<'a, T> Iterator for EitherIterator<'a, T> {
        type Item = &'a T;

        fn next(&mut self) -> Option<Self::Item> {
            match self {
                EitherIterator::Ref(value) => {
                    let result = Some(*value);
                    *self = EitherIterator::None;
                    result
                }
                EitherIterator::None => None,
            }
        }
    }

    impl<'a, L, R> IntoIterator for &'a Either<L, R> {
        type Item = &'a R;
        type IntoIter = EitherIterator<'a, R>;

        fn into_iter(self) -> Self::IntoIter {
            match &self {
                Either::Left(_) => EitherIterator::None,
                Either::Right(value) => EitherIterator::Ref(value),
            }
        }
    }

    impl<R> FromIterator<R> for Either<(), R> {
        fn from_iter<T: IntoIterator<Item = R>>(iter: T) -> Self {
            let mut iterator = iter.into_iter();
            match iterator.next() {
                None => Either::Left(()),
                Some(value) => Either::Right(value),
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

    #[test]
    fn should_map_inner_value() {
        let value: Either<String, i32> = Either::right(10);

        let result = value.into_iter().map(|&x| x * 2).collect::<Either<_,_>>();

        assert_eq!(result.is_right(), true);
        assert_eq!(result.unwrap(), 20);
    }

    #[test]
    fn should_filter_inner_value() {
        let value: Either<String, i32> = Either::right(10);

        let result = value.into_iter().filter(|&x| *x > 10).collect::<Either<_,_>>();

        assert_eq!(result.is_left(), true);
    }
}
