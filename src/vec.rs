pub mod vec {
    use std::alloc::{alloc, dealloc, Layout};
    use std::marker::PhantomData;
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

        pub fn len(&self) -> usize {
            self.size
        }

        pub fn allocated(&self) -> usize {
            self.allocated
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
                    self.allocated = INIT_SIZE;
                    self.ptr = Self::alloc(INIT_SIZE);
                    self.ptr
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

        pub fn from_elem(elem: T, count: usize) -> Self
        where
            T: Clone,
        {
            let mut vec: Vec<T> = Vec::with_capacity(count);
            unsafe {
                for i in 0..count {
                    *vec.ptr.add(i) = elem.clone();
                }
            }
            vec.size = count;
            vec
        }

        fn array_layout(capacity: usize) -> Layout {
            Layout::array::<T>(capacity).unwrap()
        }
    }

    impl<T> Drop for Vec<T> {
        fn drop(&mut self) {
            unsafe {
                if self.allocated > 0 {
                    dealloc(self.ptr as *mut u8, Self::array_layout(self.allocated));
                }
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

    pub struct VecIterator<'a, T> {
        ptr: *mut T,
        offset: usize,
        size: usize,
        phantom: PhantomData<&'a ()>,
    }

    impl<'a, T: 'a> Iterator for VecIterator<'a, T> {
        type Item = &'a T;

        fn next(&mut self) -> Option<Self::Item> {
            if self.offset < self.size {
                let result = unsafe { Option::from(&*self.ptr.add(self.offset)) };
                self.offset += 1;
                result
            } else {
                None
            }
        }
    }

    impl<'a, T> IntoIterator for &'a Vec<T> {
        type Item = &'a T;
        type IntoIter = VecIterator<'a, T>;

        fn into_iter(self) -> Self::IntoIter {
            VecIterator {
                ptr: self.ptr,
                offset: 0,
                size: self.size,
                phantom: PhantomData::default(),
            }
        }
    }

    impl<T> FromIterator<T> for Vec<T> {
        fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
            let mut result = Vec::new();
            let mut iterator = iter.into_iter();
            while let Some(value) = iterator.next() {
                result.push(value);
            }
            result
        }
    }

    #[macro_export]
    macro_rules! vec {
        () => { Vec::new() };
        ( $value:expr; $count:expr) => {
            Vec::from_elem($value, $count)
        };
        ( $($x:expr),* ) => {
            {
                use crate::count_tts;
                let mut temp = Vec::with_capacity(count_tts!($($x)*));
                $(
                    temp.push($x);
                )*
                temp
            }
        };
    }

    pub(crate) use crate::vec;
}

#[cfg(test)]
mod tests {
    use crate::vec::vec::{vec, Vec};

    #[test]
    fn should_return_empty() {
        let vec: Vec<i32> = Vec::new();

        assert_eq!(vec.len(), 0);
    }

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
        let vec: Vec<i32> = vec![1, 2, 3];

        assert_eq!(vec.allocated(), 3);
        assert_eq!(vec[0], 1);
        assert_eq!(vec[1], 2);
        assert_eq!(vec[2], 3);
    }

    #[test]
    fn should_return_range_slice() {
        let vec: Vec<i32> = vec![1, 2, 3, 4];

        let slice = &vec[1..3];

        assert_eq!(slice.len(), 2);
        assert_eq!(slice[0], 2);
        assert_eq!(slice[1], 3);
    }

    #[test]
    fn should_support_empty_vec_macro() {
        let vec: Vec<i32> = vec![];

        assert_eq!(vec.len(), 0);
    }

    #[test]
    fn should_allocate_with_macro() {
        let vec: Vec<i32> = vec![5; 7];

        assert_eq!(vec.len(), 7);
        assert_eq!(vec.len(), 7);
    }

    #[test]
    fn should_filter_out_values() {
        let vec = vec![1, 2, 3, 4, 5];

        let result = vec.into_iter().filter(|&x| *x > 3).count();

        assert_eq!(result, 2);
    }

    #[test]
    fn should_map_values() {
        let vec = vec![1, 2];

        let result: Vec<i32> = vec.into_iter().map(|&x| x * 2).collect::<Vec<_>>();

        assert_eq!(result.len(), 2);
        assert_eq!(result[0], 2);
        assert_eq!(result[1], 4);
    }
}
