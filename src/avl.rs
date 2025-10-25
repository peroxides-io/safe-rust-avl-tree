use std::{
    cmp::max,
    mem::{replace, swap},
};

type ChildNode<T> = Box<BSTNode<T>>;

// AVL tree node
#[derive(Debug)]
struct BSTNode<T: Ord> {
    left: Option<ChildNode<T>>,
    right: Option<ChildNode<T>>,
    height: i32,
    value: T,
}

impl<T: Ord> BSTNode<T> {
    pub fn new(value: T) -> ChildNode<T> {
        Box::new(Self {
            left: None,
            right: None,
            height: 0,
            value,
        })
    }

    pub fn contains(&self, value: &T) -> bool {
        if self.value == *value {
            true
        } else if *value < self.value {
            if let Some(ref node) = self.left {
                node.contains(value)
            } else {
                false
            }
        } else {
            if let Some(ref node) = self.right {
                node.contains(value)
            } else {
                false
            }
        }
    }

    // Returns true if the element inserts successfully
    fn insert_balanced(self: &mut ChildNode<T>, new_value: T) -> bool {
        if new_value == self.value {
            return false; // no-op
        }
        if new_value > self.value {
            if let Some(ref mut right) = self.right {
                right.insert_balanced(new_value);
            } else {
                self.right = Some(Self::new(new_value));
            }
        } else {
            if let Some(ref mut left) = self.left {
                left.insert_balanced(new_value);
            } else {
                self.left = Some(Self::new(new_value));
            }
        }

        self.update_height();
        self.rebalance();
        true
    }

    pub fn delete_balanced(node_opt: &mut Option<ChildNode<T>>, value: &T) -> bool {
        let deleted = match node_opt.take() {
            None => false,
            Some(mut node) => {
                if node.value > *value {
                    let del = Self::delete_balanced(&mut node.left, value);
                    node_opt.replace(node);
                    del
                } else if node.value < *value {
                    let del = Self::delete_balanced(&mut node.right, value);
                    node_opt.replace(node);
                    del
                } else {
                    match (node.left.take(), node.right.take()) {
                        (None, None) => {
                            *node_opt = None;
                        }
                        (None, Some(right)) => {
                            *node_opt = Some(right);
                        }
                        (Some(left), None) => {
                            *node_opt = Some(left);
                        }
                        (Some(left), Some(right)) => {
                            let mut temp_right = Some(right);
                            node.left = Some(left);
                            node.value = Self::take_smallest_in_subtree(&mut temp_right);
                            node.right = temp_right;

                            node_opt.replace(node);
                        }
                    }
                    true
                }
            }
        };
        if let Some(node) = node_opt {
            node.update_height();
            node.rebalance();
        }
        deleted
    }

    fn child_heights(self: &ChildNode<T>) -> (i32, i32) {
        let left_height = if let Some(ref left) = self.left {
            left.height
        } else {
            -1
        };
        let right_height = if let Some(ref right) = self.right {
            right.height
        } else {
            -1
        };
        (left_height, right_height)
    }

    // Returns true if the node's left subtree height is more than 1 away from its right subtree height
    fn is_imbalanced(self: &ChildNode<T>) -> bool {
        let (left_height, right_height) = self.child_heights();
        left_height.abs_diff(right_height) > 1
    }

    fn left_heavy(self: &ChildNode<T>) -> bool {
        let (left_height, right_height) = self.child_heights();
        left_height > right_height
    }

    fn right_heavy(self: &ChildNode<T>) -> bool {
        let (left_height, right_height) = self.child_heights();
        left_height < right_height
    }

    fn update_height(self: &mut ChildNode<T>) {
        let (left_height, right_height) = self.child_heights();
        self.height = max(left_height, right_height) + 1;
    }

    fn rotate_left(self: &mut ChildNode<T>) {
        let rl = self.right.as_mut().unwrap().left.take();

        let right = replace(&mut self.right, rl).unwrap();
        let mut s = Some(replace(self, right));
        swap(&mut self.left, &mut s);

        if let Some(node) = self.left.as_mut() {
            node.update_height();
        }
        self.update_height();
    }

    fn rotate_right(self: &mut ChildNode<T>) {
        let lr = self.left.as_mut().unwrap().right.take();

        let left = replace(&mut self.left, lr).unwrap();
        let mut s = Some(replace(self, left));
        swap(&mut self.right, &mut s);

        if let Some(node) = self.right.as_mut() {
            node.update_height();
        }
        self.update_height();
    }

    fn take_smallest_in_subtree(node_opt: &mut Option<ChildNode<T>>) -> T {
        match node_opt.take() {
            None => {
                panic!("take_smallest_in_subtree called on empty subtree");
            }
            Some(mut node) => {
                if node.left.is_some() {
                    let smallest_value = Self::take_smallest_in_subtree(&mut node.left);
                    if let Some(left) = &mut node.left {
                        left.update_height();
                        left.rebalance();
                    }
                    *node_opt = Some(node);
                    smallest_value
                } else {
                    // smallest found
                    let smallest_value = node.value;

                    *node_opt = node.right.take();
                    smallest_value
                }
            }
        }
    }

    fn rebalance(self: &mut ChildNode<T>) {
        if self.is_imbalanced() {
            if self.left_heavy() {
                if left_heavy(&self.left) {
                    self.rotate_right();
                } else {
                    self.left.as_mut().unwrap().rotate_left();
                    self.rotate_right();
                }
            } else {
                // self is right-heavy
                if right_heavy(&self.right) {
                    self.rotate_left();
                } else {
                    self.right.as_mut().unwrap().rotate_right();
                    self.rotate_left();
                }
            }
        }
    }
}

fn right_heavy<T: Ord>(node: &Option<ChildNode<T>>) -> bool {
    match node {
        None => false,
        Some(node) => node.right_heavy(),
    }
}

fn left_heavy<T: Ord>(node: &Option<ChildNode<T>>) -> bool {
    match node {
        None => false,
        Some(node) => node.left_heavy(),
    }
}

// Self-balancing AVL tree.
#[derive(Debug)]
pub struct BST<T: Ord> {
    root: Option<ChildNode<T>>,
    size: u32,
}

impl<T: Ord> BST<T> {
    pub fn new() -> Self {
        BST {
            root: None,
            size: 0,
        }
    }

    pub fn contains(&self, value: &T) -> bool {
        match &self.root {
            None => false,
            Some(node) => node.contains(value),
        }
    }

    pub fn insert(&mut self, value: T) -> bool {
        let inserted = match self.root {
            None => {
                self.root = Some(BSTNode::new(value));
                true
            }
            Some(ref mut node) => node.insert_balanced(value),
        };
        if inserted {
            self.size += 1;
        }
        inserted
    }

    pub fn delete(&mut self, value: &T) -> bool {
        let deleted = BSTNode::delete_balanced(&mut self.root, value);
        if deleted {
            self.size -= 1;
        }
        deleted
    }
}
