pub mod boxed {
    use std::alloc::Layout;
    use std::alloc::{alloc, dealloc};
    use std::fmt::{Debug, Formatter};
    use std::ops::{Deref, DerefMut};

    pub struct Box<T>(*mut T);

    impl<T> Box<T> {
        pub fn new(value: T) -> Self {
            unsafe {
                let layout = Self::layout();
                let ptr = alloc(layout) as *mut T;
                *ptr = value;
                Box(ptr)
            }
        }

        fn layout() -> Layout {
            Layout::new::<T>()
        }
    }

    impl<T> Drop for Box<T> {
        fn drop(&mut self) {
            unsafe {
                dealloc(self.0 as *mut u8, Box::<T>::layout());
            }
        }
    }

    impl<T: PartialEq> PartialEq<Self> for Box<T> {
        fn eq(&self, other: &Self) -> bool {
            unsafe { *self.0 == *other.0 }
        }
    }

    impl<T: Eq> Eq for Box<T> {}

    impl<T> Deref for Box<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            unsafe { &*self.0 }
        }
    }

    impl<T> DerefMut for Box<T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { &mut *self.0 }
        }
    }

    impl<T: Debug> Debug for Box<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            self.0.fmt(f)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::boxed;

    #[test]
    fn should_contain_value() {
        let result = boxed::Box::new(5);

        assert_eq!(*result, 5);
    }

    #[test]
    fn should_equal() {
        let first = boxed::Box::new(5);
        let second = boxed::Box::new(5);

        assert_eq!(first, second);
    }

    #[test]
    fn should_not_equal() {
        let first = boxed::Box::new(6);
        let second = boxed::Box::new(5);

        assert_ne!(first, second);
    }

    #[test]
    fn should_dereference_mutable() {
        let mut uat = boxed::Box::new(5);

        *uat = 7;

        assert_eq!(*uat, 7);
    }
}
