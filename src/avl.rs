use std::{
    cmp::max,
    mem::{replace, swap, take},
};

type ChildNode<T> = Box<BSTNode<T>>;

// AVL tree node
#[derive(Debug, Default)]
enum BSTNode<T: Ord> {
    #[default]
    Nil,
    Node {
        left: ChildNode<T>,
        right: ChildNode<T>,
        height: i32,
        value: T,
    },
}

impl<T: Ord> BSTNode<T> {
    pub fn new(value: T) -> Self {
        Self::Node {
            left: Box::new(BSTNode::Nil),
            right: Box::new(BSTNode::Nil),
            height: 0,
            value,
        }
    }

    fn contains(self: &ChildNode<T>, value: &T) -> bool {
        return match **self {
            Self::Nil => false,
            Self::Node {
                ref left,
                ref right,
                value: ref node_value,
                ..
            } => {
                if value == node_value {
                    true
                } else if value > node_value {
                    right.contains(value)
                } else {
                    left.contains(value)
                }
            }
        };
    }

    // Returns true if the element inserts successfully
    fn insert_balanced(self: &mut ChildNode<T>, new_value: T) -> bool {
        match **self {
            Self::Nil => {
                **self = Self::new(new_value);
            }
            Self::Node {
                ref mut left,
                ref mut right,
                ref value,
                ..
            } => {
                if new_value == *value {
                    return false; // no-op
                }
                if new_value > *value {
                    right.insert_balanced(new_value);
                } else {
                    left.insert_balanced(new_value);
                }
                self.update_height();
            }
        }

        self.rebalance();
        true
    }

    // returns true if value was deleted, false if not present
    pub fn delete_balanced(self: &mut ChildNode<T>, value: &T) -> bool {
        match **self {
            Self::Nil => false,
            Self::Node {
                ref mut left,
                ref mut right,
                value: ref mut node_value,
                ..
            } => {
                let deleted = if value < node_value {
                    left.delete_balanced(value)
                } else if value > node_value {
                    right.delete_balanced(value)
                } else {
                    // delete this very node
                    let has_left = !matches!(**left, BSTNode::Nil);
                    let has_right = !matches!(**right, BSTNode::Nil);

                    match (has_left, has_right) {
                        (false, false) => {
                            **self = Self::Nil;
                        }
                        (false, true) => *self = take(self.get_right()),
                        (true, false) => *self = take(self.get_left()),
                        (true, true) => {
                            let smallest_node = right.take_smallest_in_subtree();

                            if let BSTNode::Node {
                                value: smallest_value,
                                ..
                            } = *smallest_node
                            {
                                *node_value = smallest_value;
                            }
                        }
                    }
                    true
                };

                if deleted {
                    self.update_height();
                    self.rebalance();
                }
                deleted
            }
        }
    }

    // Returns true if the node's left subtree height is more than 1 away from its right subtree height
    fn is_imbalanced(self: &ChildNode<T>) -> bool {
        match **self {
            Self::Nil => false,
            Self::Node {
                ref left,
                ref right,
                ..
            } => left.get_height().abs_diff(right.get_height()) > 1,
        }
    }

    fn left_heavy(self: &ChildNode<T>) -> bool {
        match **self {
            Self::Nil => false,
            Self::Node {
                ref left,
                ref right,
                ..
            } => left.get_height() > right.get_height(),
        }
    }

    fn right_heavy(self: &ChildNode<T>) -> bool {
        match **self {
            Self::Nil => false,
            Self::Node {
                ref left,
                ref right,
                ..
            } => right.get_height() > left.get_height(),
        }
    }

    fn get_height(self: &ChildNode<T>) -> i32 {
        match **self {
            Self::Nil => -1,
            Self::Node { height, .. } => height,
        }
    }

    fn get_left<'a>(self: &'a mut ChildNode<T>) -> &'a mut ChildNode<T> {
        match **self {
            Self::Nil => panic!("tried to get left of empty BSTNode"),
            Self::Node { ref mut left, .. } => left,
        }
    }

    fn get_right<'a>(self: &'a mut ChildNode<T>) -> &'a mut ChildNode<T> {
        match **self {
            Self::Nil => panic!("tried to get left of empty BSTNode"),
            Self::Node { ref mut right, .. } => right,
        }
    }

    fn update_height(self: &mut ChildNode<T>) {
        match **self {
            Self::Nil => (),
            Self::Node {
                ref left,
                ref right,
                ref mut height,
                ..
            } => {
                *height = max(left.get_height(), right.get_height()) + 1;
            }
        }
    }

    fn rotate_left(self: &mut ChildNode<T>) {
        let rl = take(self.get_right().get_left());

        let right = replace(self.get_right(), rl);
        let mut s = replace(self, right);
        swap(self.get_left(), &mut s);

        self.get_left().update_height();
        self.update_height();
    }

    fn rotate_right(self: &mut ChildNode<T>) {
        let lr = take(self.get_left().get_right());

        let left = replace(self.get_left(), lr);
        let mut s = replace(self, left);
        swap(self.get_right(), &mut s);

        self.get_right().update_height();
        self.update_height();
    }

    fn take_smallest_in_subtree(self: &mut ChildNode<T>) -> ChildNode<T> {
        match **self {
            Self::Nil => panic!("empty subtree"),
            Self::Node { ref mut left, .. } => {
                if let Self::Nil = **left {
                    // smallest found
                    let right_child = take(self.get_right());

                    let smallest_node = take(self);

                    **self = *right_child;
                    smallest_node
                } else {
                    let smallest = left.take_smallest_in_subtree();
                    self.update_height();
                    self.rebalance();
                    smallest
                }
            }
        }
    }

    fn rebalance(self: &mut ChildNode<T>) {
        if !self.is_imbalanced() {
            return;
        }

        if self.left_heavy() {
            let left = self.get_left();
            if left.left_heavy() {
                self.rotate_right();
            } else {
                left.rotate_left();
                self.rotate_right();
            }
        } else {
            // self is right-heavy
            let right = self.get_right();
            if right.right_heavy() {
                self.rotate_left();
            } else {
                right.rotate_right();
                self.rotate_left();
            }
        }
    }
}

// Self-balancing AVL tree.
#[derive(Debug)]
pub struct BST<T: Ord> {
    root: ChildNode<T>,
    size: u32,
}

impl<T: Ord> BST<T> {
    pub fn new() -> Self {
        BST {
            root: Box::new(BSTNode::Nil),
            size: 0,
        }
    }

    pub fn contains(&self, value: &T) -> bool {
        self.root.contains(value)
    }

    pub fn insert(&mut self, value: T) -> bool {
        let inserted = self.root.insert_balanced(value);
        if inserted {
            self.size += 1;
        }
        inserted
    }

    pub fn delete(&mut self, value: &T) -> bool {
        let deleted = self.root.delete_balanced(value);
        if deleted {
            self.size -= 1;
        }
        deleted
    }
}
