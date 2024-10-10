/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/

use std::cmp::Ordering;
use std::fmt::Debug;
use std::thread::sleep;

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
        //TODO
        match &mut self.root {
            Some(val)=>{
                if value>val.value{
                    match &mut val.right {
                        Some(v)=>{
                            //val.right.insert(value);
                            val.insert(value);
                        },
                        None=>{
                            val.right=Some(Box::new(TreeNode{value:value,left:None,right:None}));
                        }
                    }
                }else if value<val.value{
                    match &mut val.left {
                        Some(v)=>{
                            val.insert(value);
                        },
                        None=>{
                            val.left=Some(Box::new(TreeNode{value:value,left:None,right:None}));
                        }                    }
                    //
                }
            }
            None=>{
                let new=Some(Box::new(TreeNode{value:value,left:None,right:None}));
                self.root=new;
            }
        }
    }

    // Search for a value in the BST
    fn search(& self, value: T) -> bool {
        //TODO
        let mut z=self.root.as_ref();
        match z {
            Some(v)=>{
                if v.value==value{
                    return true;
                }
            }
            None =>{
                return false;
            }
        }
        while !z.unwrap().left.is_none()||!z.unwrap().right.is_none(){
            if value>z.unwrap().value{
                match &z.unwrap().right {
                    Some(v)=>{
                        if v.value==value{
                            return true;
                        }else{
                            z=Some(v);
                        }
                    },
                    None=>{
                        return false;
                    }
                }
            }else{
                match &z.unwrap().left {
                    Some(v)=>{
                        if v.value==value{
                            return true;
                        }else{
                            z=Some(v);
                        }
                    },
                    None=>{
                        return false;
                    }
                }
            }
        }

        return false
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    pub fn insert(&mut self, value: T) {
        //TODO
        if self.value<value{
            print!("1");
            match &mut self.right{
                Some( v)=>{
                   // print!("{}",v.value);
                    v.insert(value);
                },
                None=>{
                    self.right=Some((Box::new(TreeNode{value:value,left:None,right:None})));
                }
            }
        }else if self.value>value{
            match &mut self.left {
                Some( v)=>{
                    v.insert(value);
                },
                None=>{
                    self.left =Some((Box::new(TreeNode{value:value,left:None,right:None})));
                }

            }
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

