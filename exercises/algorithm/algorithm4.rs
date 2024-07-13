/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/

use std::cmp::Ordering;
use std::fmt::Debug;


#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }

    // Insert a value into the tree
    fn insert(&mut self, value: T) {
        match &self.value.cmp(&value) {
            Ordering::Less => {
                match &mut self.left {
                    Some(node) => node.insert(value),
                    _ => self.left = Some(Box::new(TreeNode::<T>::new(value))),
                }
            },
            Ordering::Greater => {
                match &mut self.right {
                    Some(node) => node.insert(value),
                    _ => self.right = Some(Box::new(TreeNode::<T>::new(value))),
                }
            },
            _ => {},
        }
    }

    // Search for a value in the tree
    fn search(&self, value: T) -> bool {
        match &self.value.cmp(&value) {
            Ordering::Less => self.left.as_ref().map_or(false, |n| n.search(value)),
            Ordering::Greater => self.right.as_ref().map_or(false, |n| n.search(value)),
            _ => true,
        }
    }
}


impl<T> BinarySearchTree<T>
where
    T: Ord,
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        match &mut self.root {
            Some(root) => root.insert(value),
            _ => self.root = Some(Box::new(TreeNode::<T>::new(value))),
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        match &self.root {
            Some(root) => root.search(value),
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        
        assert_eq!(bst.search(1), false);

        
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        
        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        
        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        
        bst.insert(1);
        bst.insert(1);

        
        assert_eq!(bst.search(1), true);

        
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    


