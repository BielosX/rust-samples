pub mod rc {
    use std::alloc::{alloc, dealloc};
    use std::alloc::Layout;
    use std::ops::Deref;

    pub struct Rc<T>(*mut T, *mut usize);

    impl<T> Rc<T> {
        pub fn new(value: T) -> Self {
            unsafe {
                let ptr = alloc(Self::layout()) as *mut T;
                let counter = alloc(Self::counter_layout()) as *mut usize;
                *ptr = value;
                *counter = 1;
                Rc(ptr, counter)
            }
        }

        pub fn strong_count(this: &Self) -> usize {
            unsafe { *this.1 }
        }

        fn counter_layout() -> Layout {
            Layout::new::<usize>()
        }

        fn layout() -> Layout {
            Layout::new::<T>()
        }

        pub fn clone(old: &Self) -> Self {
            old.clone()
        }
    }

    impl<T> Deref for Rc<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            unsafe { &*self.0 }
        }
    }

    impl<T> Clone for Rc<T> {
        fn clone(&self) -> Self {
            unsafe {
                *self.1 += 1;
                Rc(self.0, self.1)
            }
        }
    }

    impl<T> Drop for Rc<T> {
        fn drop(&mut self) {
            unsafe {
                *self.1 -= 1;
                if *self.1 == 0 {
                    dealloc(self.0 as *mut u8, Rc::<T>::layout());
                    dealloc(self.1 as *mut u8, Rc::<T>::counter_layout());
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::rc::rc;

    #[test]
    fn should_dereference_properly() {
        let uat = rc::Rc::new(7);

        assert_eq!(*uat, 7);
    }

    #[test]
    fn should_decrement_counter_on_scope_exit() {
        let first = rc::Rc::new(7);

        {
            let second = rc::Rc::clone(&first);

            assert_eq!(rc::Rc::strong_count(&first), 2);
            assert_eq!(rc::Rc::strong_count(&second), 2);
        }

        assert_eq!(rc::Rc::strong_count(&first), 1);
    }
}
