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

    pub fn push(&mut self, val: T) {
        assert_ne!(std::mem::size_of::<T>(), 0, "no zero sized types!");

        if self.capacity == 0 {
            // hardcode layout to be 4 x size_of<T>
            let layout = Layout::array::<T>(4).expect("could not allocate");
            let ptr = unsafe { alloc::alloc(layout) } as *mut T;
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
}
