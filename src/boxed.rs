mod boxed {
    use std::alloc::{GlobalAlloc, Layout, System};
    use std::fmt::{Debug, Formatter};
    use std::ops::{Deref, DerefMut};

    pub struct Box<T>(*mut T, Layout);

    impl<T> Box<T> {
        pub fn new(value: T) -> Self {
            unsafe {
                let layout = Layout::new::<T>();
                let ptr = System.alloc(layout) as *mut T;
                *ptr = value;
                Box(ptr, layout)
            }
        }
    }

    impl<T> Drop for Box<T> {
        fn drop(&mut self) {
            unsafe {
                System.dealloc(self.0 as *mut u8, self.1);
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
}
