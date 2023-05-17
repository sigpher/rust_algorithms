use std::fmt::{Debug, Display};

type Link<T> = Option<Box<BinaryTree<T>>>;

#[derive(Debug, Clone)]
pub struct BinaryTree<T> {
    pub key: T,
    pub left: Link<T>,
    pub right: Link<T>,
}

impl<T: Clone> BinaryTree<T> {
    pub fn new(key: T) -> Self {
        BinaryTree {
            key,
            left: None,
            right: None,
        }
    }

    pub fn insert_left_tree(&mut self, key: T) {
        if self.left.is_none() {
            let node = BinaryTree::new(key);
            self.left = Some(Box::new(node));
        } else {
            let mut node = BinaryTree::new(key);
            node.left = self.left.take();
            self.left = Some(Box::new(node));
        }
    }

    pub fn insert_right_tree(&mut self, key: T) {
        if self.right.is_none() {
            let node = BinaryTree::new(key);
            self.right = Some(Box::new(node));
        } else {
            let mut node = BinaryTree::new(key);
            node.right = self.right.take();
            self.right = Some(Box::new(node));
        }
    }

    pub fn get_left(&self) -> Link<T> {
        self.left.clone()
    }

    pub fn get_right(&self) -> Link<T> {
        self.right.clone()
    }

    pub fn get_key(&self) -> T {
        self.key.clone()
    }

    pub fn set_key(&mut self, key: T) {
        self.key = key
    }
}
