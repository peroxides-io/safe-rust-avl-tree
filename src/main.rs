mod avl;

use crate::avl::BST;

// Basic use of BST functions
fn main() {
    let mut bst = BST::new();
    for i in 1..=63 {
        bst.insert(i);
    }
    for i in 10..=30 {
        bst.delete(&i);
    }
    for i in 1..=9 {
        assert!(bst.contains(&i));
    }
    for i in 10..=30 {
        assert!(!bst.contains(&i));
    }
    for i in 31..=63 {
        assert!(bst.contains(&i));
    }
    bst.delete(&1);
    bst.delete(&3);
    bst.delete(&7);
    println!("BST operation successful!")
}
