pub mod vec {
    use std::alloc::{alloc, dealloc, Layout};
    use std::ops::{Index, IndexMut, Range};
    use std::ptr::null_mut;
    use std::slice;

    const INIT_SIZE: usize = 4;

    pub struct Vec<T> {
        ptr: *mut T,
        size: usize,
        allocated: usize,
    }

    impl<T> Vec<T> {
        pub fn new() -> Self {
            Vec {
                ptr: null_mut(),
                size: 0,
                allocated: 0,
            }
        }

        pub fn with_capacity(capacity: usize) -> Self {
            unsafe {
                let ptr = Self::alloc(capacity);
                Vec {
                    ptr,
                    size: 0,
                    allocated: capacity,
                }
            }
        }

        unsafe fn alloc(capacity: usize) -> *mut T {
            alloc(Self::array_layout(capacity)) as *mut T
        }

        pub fn push(&mut self, value: T) {
            unsafe {
                let ptr = if self.allocated == 0 {
                    Self::alloc(INIT_SIZE)
                } else {
                    self.ptr
                };
                if self.size == self.allocated {
                    let new_size = self.allocated << 2;
                    let new_ptr = Self::alloc(new_size);
                    ptr.copy_to_nonoverlapping(new_ptr, self.size);
                }
                *(ptr.add(self.size)) = value;
                self.size += 1;
            }
        }

        fn array_layout(capacity: usize) -> Layout {
            Layout::array::<T>(capacity).unwrap()
        }
    }

    impl<T> Drop for Vec<T> {
        fn drop(&mut self) {
            unsafe {
                dealloc(self.ptr as *mut u8, Self::array_layout(self.allocated));
            }
        }
    }

    impl<T> Index<usize> for Vec<T> {
        type Output = T;

        fn index(&self, index: usize) -> &Self::Output {
            unsafe { &*self.ptr.add(index) }
        }
    }

    impl<T> Index<Range<usize>> for Vec<T> {
        type Output = [T];

        fn index(&self, index: Range<usize>) -> &Self::Output {
            unsafe { slice::from_raw_parts(self.ptr.add(index.start), index.end - index.start) }
        }
    }

    impl<T> IndexMut<usize> for Vec<T> {
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            unsafe { &mut *self.ptr.add(index) }
        }
    }

    #[macro_export]
    macro_rules! vec {
        ( $($x:expr),* ) => {
            {
                let mut temp = Vec::new();
                $(
                    temp.push($x);
                )*
                temp
            }
        };
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn should_return_value_at_index() {
        let mut vec: Vec<i32> = Vec::new();
        vec.push(1);
        vec.push(2);

        assert_eq!(vec[0], 1);
        assert_eq!(vec[1], 2);
    }

    #[test]
    fn should_mutate_value_at_index() {
        let mut vec: Vec<i32> = Vec::new();
        vec.push(1);
        vec.push(2);

        vec[1] = 17;

        assert_eq!(vec[0], 1);
        assert_eq!(vec[1], 17);
    }

    #[test]
    fn should_construct_vector() {
        let vec = vec![1, 2, 3];

        assert_eq!(vec[0], 1);
        assert_eq!(vec[1], 2);
        assert_eq!(vec[2], 3);
    }

    #[test]
    fn should_return_range_slice() {
        let vec = vec![1, 2, 3, 4];

        let slice = &vec[1..3];

        assert_eq!(slice.len(), 2);
        assert_eq!(slice[0], 2);
        assert_eq!(slice[1], 3);
    }
}
