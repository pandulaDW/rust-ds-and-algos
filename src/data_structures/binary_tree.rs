#![allow(dead_code)]

/// An ordered collection of trees
#[derive(Debug)]
pub enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>),
}

impl<T: Ord> BinaryTree<T> {
    /// adds a new node to the tree
    pub fn add(&mut self, value: T) {
        match self {
            BinaryTree::Empty => {
                *self = BinaryTree::NonEmpty(Box::new(TreeNode {
                    element: value,
                    left: BinaryTree::Empty,
                    right: BinaryTree::Empty,
                }));
            }
            BinaryTree::NonEmpty(ref mut node) => {
                if value <= node.element {
                    node.left.add(value);
                } else {
                    node.right.add(value);
                }
            }
        }
    }

    // creates an instance of TreeIter and walks the first left path before
    // returning the iterator
    pub fn iter(&self) -> TreeIter<T> {
        let mut iter = TreeIter {
            unvisited: Vec::new(),
        };
        iter.push_left_edge(self);
        iter
    }
}

impl<'a, T: 'a> IntoIterator for &'a BinaryTree<T>
where
    T: Ord,
{
    type Item = &'a T;
    type IntoIter = TreeIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// A node of the binary tree
#[derive(Debug)]
pub struct TreeNode<T> {
    element: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}

/// The state of an in-order traversal of a "BinaryTree"
pub struct TreeIter<'a, T> {
    unvisited: Vec<&'a TreeNode<T>>,
}

impl<'a, T> TreeIter<'a, T>
where
    T: Ord,
{
    fn push_left_edge(&mut self, mut tree: &'a BinaryTree<T>) {
        while let BinaryTree::NonEmpty(ref node) = *tree {
            self.unvisited.push(node);
            tree = &node.left;
        }
    }
}

impl<'a, T> Iterator for TreeIter<'a, T>
where
    T: Ord,
{
    type Item = &'a T;
    fn next(&mut self) -> Option<&'a T> {
        let node = self.unvisited.pop()?;
        self.push_left_edge(&node.right);
        Some(&node.element)
    }
}
