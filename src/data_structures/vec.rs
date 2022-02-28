#![allow(dead_code)]

use std::alloc::{self, Layout};
use std::ptr::NonNull;

pub struct MyVec<T> {
    ptr: NonNull<T>,
    len: usize,
    capacity: usize,
}

impl<T> MyVec<T> {
    pub fn new() -> Self {
        Self {
            ptr: NonNull::dangling(),
            len: 0,
            capacity: 0,
        }
    }

    pub fn push(&mut self, item: T) {
        assert_ne!(std::mem::size_of::<T>(), 0, "no zero sized types!");

        if self.capacity == 0 {
            // hardcode layout to be 4 x size_of<T>
            let layout = Layout::array::<T>(4).expect("could not allocate memory");
            let ptr = unsafe { alloc::alloc(layout) } as *mut T;
            let ptr = NonNull::new(ptr).expect("could not allocate memory");
            self.ptr = ptr;
            self.capacity = 4;

            // SAFE since, pointer is non-null and we have just allocated enough space for this item
            unsafe { self.ptr.as_ptr().write(item) };
            self.len = 1;
        } else if self.len < self.capacity {
            // check if offset is valid
            let offset = self
                .len
                .checked_mul(std::mem::size_of::<T>())
                .expect("cannot reach memory location");
            assert!(offset < isize::MAX as usize, "wrapped isize");

            unsafe {
                self.ptr.as_ptr().add(self.len).write(item);
                self.len += 1;
            }
        } else {
            todo!();
        }
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

#[cfg(test)]
mod tests {
    use super::MyVec;

    #[test]
    fn test_vec_initialize_correctly() {
        let vec = MyVec::<usize>::new();

        assert_eq!(vec.capacity(), 0);
        assert_eq!(vec.len(), 0);
    }

    #[test]
    #[should_panic(expected = "no zero sized types!")]
    fn test_unit_size_panics() {
        let mut vec = MyVec::<()>::new();
        vec.push(());
    }

    #[test]
    fn test_push_elements_without_reallocation() {
        let mut vec = MyVec::new();

        vec.push(10);
        vec.push(12);
        vec.push(13);
        vec.push(14);

        assert_eq!(4, vec.capacity);
        assert_eq!(4, vec.len);
    }
}
