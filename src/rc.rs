pub mod rc {
    use std::alloc::Layout;
    use std::alloc::{alloc, dealloc};
    use std::fmt::{Debug, Display, Formatter};
    use std::ops::Deref;
    use std::ptr::null_mut;

    #[derive(Clone, Debug)]
    struct Counter(*mut usize, *mut usize);

    impl Counter {
        fn new() -> Self {
            unsafe {
                let strong_counter = alloc(Self::counter_layout()) as *mut usize;
                let weak_counter = alloc(Self::counter_layout()) as *mut usize;
                Counter(strong_counter, weak_counter)
            }
        }

        fn empty() -> Self {
            Counter(null_mut(), null_mut())
        }

        fn counter_layout() -> Layout {
            Layout::new::<usize>()
        }

        unsafe fn inc_strong(&self) {
            if !self.0.is_null() {
                *self.0 += 1;
            }
        }

        unsafe fn inc_weak(&self) {
            if !self.1.is_null() {
                *self.1 += 1;
            }
        }

        unsafe fn dec_strong(&self) {
            if !self.0.is_null() {
                *self.0 -= 1;
            }
        }

        unsafe fn dec_weak(&self) {
            if !self.1.is_null() {
                *self.1 -= 1;
            }
        }

        fn is_strong_positive(&self) -> bool {
            unsafe {
                if !self.0.is_null() {
                    *self.0 > 0
                } else {
                    false
                }
            }
        }
    }

    impl Display for Counter {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            unsafe {
                let strong = if self.0.is_null() {
                    String::from("null")
                } else {
                    (*self.0).to_string()
                };
                let weak = if self.1.is_null() {
                    String::from("null")
                } else {
                    (*self.1).to_string()
                };
                write!(f, "({}, {})", strong, weak)
            }
        }
    }

    impl Drop for Counter {
        fn drop(&mut self) {
            unsafe {
                if !(self.0.is_null() && self.1.is_null()) {
                    if *self.0 == 0 && *self.1 == 0 {
                        dealloc(self.0 as *mut u8, Self::counter_layout());
                        dealloc(self.1 as *mut u8, Self::counter_layout());
                    }
                }
            }
        }
    }

    #[derive(Debug)]
    pub struct Rc<T>(*mut T, Counter);

    impl<T> Rc<T> {
        pub fn new(value: T) -> Self {
            unsafe {
                let ptr = alloc(Self::layout()) as *mut T;
                let counter = Counter::new();
                *ptr = value;
                counter.inc_strong();
                Rc(ptr, counter)
            }
        }

        pub fn strong_count(this: &Self) -> usize {
            unsafe { *this.1 .0 }
        }

        pub fn weak_count(this: &Self) -> usize {
            unsafe { *this.1 .1 }
        }

        fn layout() -> Layout {
            Layout::new::<T>()
        }

        pub fn clone(old: &Self) -> Self {
            old.clone()
        }

        pub fn downgrade(this: &Self) -> Weak<T> {
            unsafe {
                this.1.inc_weak();
                Weak(this.0, this.1.clone())
            }
        }
    }

    impl<T: Eq> PartialEq<Self> for Rc<T> {
        fn eq(&self, other: &Self) -> bool {
            unsafe { *self.0 == *other.0 }
        }
    }

    impl<T: Eq> Eq for Rc<T> {}

    impl<T: Display> Display for Rc<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            unsafe {
                let value = if self.0.is_null() {
                    String::from("null")
                } else {
                    (*self.0).to_string()
                };
                write!(f, "({}, {})", value, self.1)
            }
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
                self.1.inc_strong();
                Rc(self.0, self.1.clone())
            }
        }
    }

    impl<T> Drop for Rc<T> {
        fn drop(&mut self) {
            unsafe {
                self.1.dec_strong();
                if !self.1.is_strong_positive() {
                    dealloc(self.0 as *mut u8, Rc::<T>::layout());
                }
            }
        }
    }

    pub struct Weak<T>(*mut T, Counter);

    impl<T> Weak<T> {
        pub fn new() -> Self {
            Weak(null_mut(), Counter::empty())
        }

        pub fn strong_count(&self) -> usize {
            unsafe { *self.1 .0 }
        }

        pub fn weak_count(&self) -> usize {
            unsafe { *self.1 .1 }
        }

        pub fn upgrade(&self) -> Option<Rc<T>> {
            unsafe {
                if self.1.is_strong_positive() {
                    self.1.inc_strong();
                    Some(Rc(self.0, self.1.clone()))
                } else {
                    None
                }
            }
        }
    }

    impl<T> Drop for Weak<T> {
        fn drop(&mut self) {
            unsafe {
                self.1.dec_weak();
            }
        }
    }

    impl<T> Clone for Weak<T> {
        fn clone(&self) -> Self {
            unsafe {
                self.1.inc_weak();
                Weak(self.0, self.1.clone())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::rc::rc;
    use crate::rc::rc::{Rc, Weak};

    #[test]
    fn should_dereference_properly() {
        let uat = rc::Rc::new(7);

        assert_eq!(*uat, 7);
    }

    #[test]
    fn should_decrement_counter_on_scope_exit() {
        let first = Rc::new(7);

        {
            let second = Rc::clone(&first);

            assert_eq!(rc::Rc::strong_count(&first), 2);
            assert_eq!(rc::Rc::strong_count(&second), 2);
        }

        assert_eq!(rc::Rc::strong_count(&first), 1);
    }

    #[test]
    fn should_return_zero_when_no_weak() {
        let reference = Rc::new(7);

        {
            let weak = Rc::downgrade(&reference);

            assert_eq!(Rc::strong_count(&reference), 1);
            assert_eq!(Rc::weak_count(&reference), 1);
            assert_eq!(weak.strong_count(), 1);
            assert_eq!(weak.weak_count(), 1);
        }

        assert_eq!(Rc::weak_count(&reference), 0);
    }

    #[test]
    fn should_return_zero_strong_when_no_rc() {
        let weak: Weak<i32>;

        {
            let reference = Rc::new(7);
            weak = Rc::downgrade(&reference).clone();
        }

        assert_eq!(weak.strong_count(), 0);
        assert_eq!(weak.weak_count(), 1);
    }

    #[test]
    fn should_return_none_on_upgrade_when_no_strong() {
        let weak = Weak::<i32>::new();

        assert_eq!(weak.upgrade(), None);
    }

    #[test]
    fn should_return_reference_on_upgrade() {
        let reference = Rc::<i32>::new(7);
        let weak = Rc::downgrade(&reference);

        let new_reference = weak.upgrade();

        assert_eq!(new_reference.is_some(), true);
    }
}
