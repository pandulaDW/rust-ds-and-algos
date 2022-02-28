#![allow(dead_code)]

use std::fmt::{Debug, Display};

#[derive(Debug)]
pub enum List<T> {
    Empty,
    NonEmpty(Box<Node<T>>),
}

impl<T: Debug> List<T> {
    pub fn new() -> Self {
        List::Empty
    }

    pub fn append(&mut self, value: T) {
        match self {
            List::Empty => {
                *self = List::NonEmpty(Box::new(Node {
                    element: value,
                    next: List::Empty,
                }));
            }
            List::NonEmpty(ref mut node) => {
                node.next.append(value);
            }
        }
    }

    pub fn pop(&self) {
        match self {
            List::Empty => {}
            List::NonEmpty(node) => {
                println!("{:?}", node.element);
                node.next.pop();
            }
        }
    }

    fn join_elements(&self, joiner: &mut Vec<String>) {
        match self {
            List::Empty => {}
            List::NonEmpty(node) => {
                joiner.push(format!("{:?}", node.element));
                node.next.join_elements(joiner);
            }
        }
    }
}

impl<T: Debug> Display for List<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut v = vec![];
        self.join_elements(&mut v);
        write!(f, "{}", v.join(" -> "))
    }
}

#[derive(Debug)]
pub struct Node<T> {
    element: T,
    next: List<T>,
}
