use std::fmt::Debug;
use std::cmp::Ordering::*;
use std::mem::{self, transmute};

#[derive(Debug)]
pub struct Node<K> {
    key: K,
    left: Tree<K>,
    right: Tree<K>,
}

#[derive(Debug)]
pub enum Tree<K> {
    Node(Box<Node<K>>),
    Empty
}

enum Strategy {
    Any,
    Before,
    After
}

impl<K: Ord+Debug> Tree<K> {
    fn leaf(key: K) -> Tree<K> {
        Tree::Node(Box::new(Node {
            key: key,
            left: Tree::Empty,
            right: Tree::Empty
        }))
    }

    fn take(&mut self) -> Self {
        mem::replace(self, Tree::Empty)
    }

    fn find(&self, key: &K, strategy: Strategy) -> &Self {
        let mut cur = self;

        while let Tree::Node(ref node) = *cur {
            match node.key.cmp(key) {
                Less    => cur = &node.right,
                Greater => cur = &node.left,
                Equal   => {
                    match strategy {
                        Strategy::Any    => break,
                        Strategy::Before => cur = &node.left,
                        Strategy::After  => cur = &node.right
                    }
                }
            }
        }
        
        cur
    }

    fn find_mut(&mut self, key: &K, strategy: Strategy) -> &mut Self {
        unsafe {
            transmute::<*mut Self, &mut Self>(transmute::<&Self, *mut Self>(self.find(key, strategy)))
        }
    }

    fn first(&self) -> &Self {
        let mut cur = self;

        while let Tree::Node(ref node) = *cur {
            match node.left {
                Tree::Node(_) => cur = &node.left,
                Tree::Empty   => break
            }
        }

        cur
    }

    fn first_mut(&mut self) -> &mut Self {
        unsafe {
            transmute::<*mut Self, &mut Self>(transmute::<&Self, *mut Self>(self.first()))
        }
    }

    pub fn has(&self, key: K) -> bool {
        match *self.find(&key, Strategy::Any) {
            Tree::Node(_) => true,
            Tree::Empty => false
        }
    }

    pub fn insert(&mut self, key: K) {
        let tree = self.find_mut(&key, Strategy::After);

        match *tree {
            Tree::Node(_) => unreachable!(),
            Tree::Empty => *tree = Tree::leaf(key)
        }
    }

    pub fn delete(&mut self, key: K) {
        let tree = self.find_mut(&key, Strategy::Any);
        
        *tree = if let Tree::Node(mut node) = tree.take() {
            if let Tree::Node(_) = node.right {
                let left = node.left.take();//first_mut();
                
                let mut right = node.right.take();//first_mut();

                let succ = {
                    let mut succ_slot = right.first_mut();
                    let mut succ = succ_slot.take();

                    *succ_slot = if let Tree::Node(ref mut node) = succ {
                        node.right.take()
                    } else {
                        unreachable!();
                    };

                    succ
                };
                
                if let Tree::Node(succ_node) = succ {
                    node.key = succ_node.key;
                    node.left = left;
                    node.right = right;
                } else {
                    unreachable!();
                }
                // node.left = left;
                // node.right = right.right

                Tree::Node(node)
            } else {
                node.left
            }
        } else {
            return; // The key wasn't in the tree
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert() {
        let mut bst = Tree::Empty;

        bst.insert(5);
        bst.insert(2);

        assert_eq!(
            (0..10).map(|i| bst.has(i)).collect::<Vec<_>>(),
            [false, false, true, false, false, true, false, false, false, false]
        );

        bst.insert(3);
        bst.insert(1);
        bst.insert(7);

        assert_eq!(
            (0..10).map(|i| bst.has(i)).collect::<Vec<_>>(),
            [false, true, true, true, false, true, false, true, false, false]
        );
    }

    #[test]
    fn insert_and_delete() {
        let mut bst = Tree::Empty;

        const F: bool = false;
        const T: bool = true;

        bst.insert(7);
        bst.insert(3);
        bst.insert(11);
        bst.insert(1);
        bst.insert(5);
        bst.insert(9);
        bst.insert(13);
        assert_eq!((0..15).map(|i| bst.has(i)).collect::<Vec<_>>(), [F, T, F, T, F, T, F, T, F, T, F, T, F, T, F]);

        bst.insert(0);
        bst.insert(2);
        bst.insert(4);
        bst.insert(6);
        bst.insert(8);
        bst.insert(10);
        bst.insert(12);
        bst.insert(14);
        assert_eq!((0..15).map(|i| bst.has(i)).collect::<Vec<_>>(), [T, T, T, T, T, T, T, T, T, T, T, T, T, T, T]);

        bst.delete(7);
        assert_eq!((0..15).map(|i| bst.has(i)).collect::<Vec<_>>(), [T, T, T, T, T, T, T, F, T, T, T, T, T, T, T]);
        bst.delete(3);
        assert_eq!((0..15).map(|i| bst.has(i)).collect::<Vec<_>>(), [T, T, T, F, T, T, T, F, T, T, T, T, T, T, T]);
        bst.delete(11);
        assert_eq!((0..15).map(|i| bst.has(i)).collect::<Vec<_>>(), [T, T, T, F, T, T, T, F, T, T, T, F, T, T, T]);
        bst.delete(1);
        assert_eq!((0..15).map(|i| bst.has(i)).collect::<Vec<_>>(), [T, F, T, F, T, T, T, F, T, T, T, F, T, T, T]);
        bst.delete(5);
        assert_eq!((0..15).map(|i| bst.has(i)).collect::<Vec<_>>(), [T, F, T, F, T, F, T, F, T, T, T, F, T, T, T]);
        bst.delete(9);
        assert_eq!((0..15).map(|i| bst.has(i)).collect::<Vec<_>>(), [T, F, T, F, T, F, T, F, T, F, T, F, T, T, T]);
        bst.delete(13);
        assert_eq!((0..15).map(|i| bst.has(i)).collect::<Vec<_>>(), [T, F, T, F, T, F, T, F, T, F, T, F, T, F, T]);
        bst.delete(0);
        assert_eq!((0..15).map(|i| bst.has(i)).collect::<Vec<_>>(), [F, F, T, F, T, F, T, F, T, F, T, F, T, F, T]);
        bst.delete(2);
        assert_eq!((0..15).map(|i| bst.has(i)).collect::<Vec<_>>(), [F, F, F, F, T, F, T, F, T, F, T, F, T, F, T]);
        bst.delete(4);
        assert_eq!((0..15).map(|i| bst.has(i)).collect::<Vec<_>>(), [F, F, F, F, F, F, T, F, T, F, T, F, T, F, T]);
        bst.delete(6);
        assert_eq!((0..15).map(|i| bst.has(i)).collect::<Vec<_>>(), [F, F, F, F, F, F, F, F, T, F, T, F, T, F, T]);
        bst.delete(8);
        assert_eq!((0..15).map(|i| bst.has(i)).collect::<Vec<_>>(), [F, F, F, F, F, F, F, F, F, F, T, F, T, F, T]);
        bst.delete(10);
        assert_eq!((0..15).map(|i| bst.has(i)).collect::<Vec<_>>(), [F, F, F, F, F, F, F, F, F, F, F, F, T, F, T]);
        bst.delete(12);
        assert_eq!((0..15).map(|i| bst.has(i)).collect::<Vec<_>>(), [F, F, F, F, F, F, F, F, F, F, F, F, F, F, T]);
        bst.delete(14);
        assert_eq!((0..15).map(|i| bst.has(i)).collect::<Vec<_>>(), [F, F, F, F, F, F, F, F, F, F, F, F, F, F, F]);
    }

    #[test]
    fn insert_and_delete_duplicates() {
        let mut bst = Tree::Empty;

        bst.insert(0);
        bst.insert(1);
        bst.insert(0);
        bst.insert(1);
        bst.insert(0);
        bst.insert(1);

        assert_eq!(
            (0..2).map(|i| bst.has(i)).collect::<Vec<_>>(),
            [true, true]
        );

        // println!("{:#?}", bst);
        // println!("====================================================");

        bst.delete(0);
        // println!("{:#?}", bst);
        // println!("########################################################");
        bst.delete(0);
        bst.delete(1);

        assert_eq!(
            (0..2).map(|i| bst.has(i)).collect::<Vec<_>>(),
            [true, true]
        );

        bst.delete(0);

        assert_eq!(
            (0..2).map(|i| bst.has(i)).collect::<Vec<_>>(),
            [false, true]
        );
    }
}
