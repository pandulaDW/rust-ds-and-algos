#![allow(dead_code)]

use std::fmt::{self, Debug};

pub struct Stack<T: Debug> {
    data: Vec<T>,
}

impl<T> Stack<T>
where
    T: Debug,
{
    pub fn new() -> Stack<T> {
        let v: Vec<T> = vec![];
        Stack { data: v }
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn push(&mut self, item: T) {
        self.data.push(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }
}

impl<T> fmt::Display for Stack<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let stack_str = self
            .data
            .iter()
            .map(|el| format!("{:?}", el))
            .collect::<Vec<String>>()
            .join(" -> ");

        write!(f, "{}", stack_str)
    }
}
