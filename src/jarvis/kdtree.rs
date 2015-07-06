use std::cmp::Ord;
use std::ops::Index;

pub trait KdData: Ord + Eq + Index<usize> {
    fn dimensions(&self) -> usize;
}

pub struct KdTree<T> {
    root: Option<Box<Node<T>>>,
}

struct Node<T> {
    data: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(data: T) -> Node<T> {
        Node {
            data: data,
            left: None,
            right: None,
        }
    }

    fn insert_left(&mut self, node: Node<T>) {
        self.left = Some(box node);
    }

    fn insert_right(&mut self, node: Node<T>) {
        self.right = Some(box node);
    }

    fn left(&self) -> Option<&Node<T>> {
        if let Some(ref node) = self.left {
            Some(node)
        } else {
            None
        }
    }

    fn right(&self) -> Option<&Node<T>> {
        if let Some(ref node) = self.right {
            Some(node)
        } else {
            None
        }
    }

    fn left_mut(&mut self) -> Option<&mut Node<T>> {
        if let Some(ref mut node) = self.left {
            Some(node)
        } else {
            None
        }
    }

    fn right_mut(&mut self) -> Option<&mut Node<T>> {
        if let Some(ref mut node) = self.right {
            Some(node)
        } else {
            None
        }
    }
}

impl<T> KdTree<T>
    where T: KdData
{
    pub fn new(dataset: &[T]) -> KdTree<T> {
        // Filter empty datasets and determine dimensionality if non-empty.
        let dim = if let Some(elt) = dataset.first() {
            elt.dimensions()
        } else {
            return KdTree::default();
        };
        unimplemented!()
    }
}

impl<T> Default for KdTree<T>
    where T: KdData
{
    // Emtry tree with type information.
    fn default() -> KdTree<T> {
        KdTree {
            root: None
        }
    }
}
