use std::fmt::Debug;
use std::cmp::Ordering::*;
use std::mem::{self, transmute};

enum FindStrategy {
    Any,
    Before,
    After
}

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

    fn find_mut(&mut self, key: &K, strategy: FindStrategy) -> &mut Self {
        let mut cur = self;

        loop {
            let temp = cur;
            if temp.inner.is_some() {
                cur = match temp.inner.as_mut().unwrap().key.cmp(key) {
                    Less    => &mut temp.inner.as_mut().unwrap().right,
                    Greater => &mut temp.inner.as_mut().unwrap().left,
                    Equal   => {
                        match strategy {
                            FindStrategy::Any    => temp,
                            FindStrategy::Before => &mut temp.inner.as_mut().unwrap().left,
                            FindStrategy::After  => &mut temp.inner.as_mut().unwrap().right
                        }
                    }
                };
            } else {
                return temp;
            }
        }
        
        cur
    }

    fn first_mut(&mut self) -> &mut Self {
        let mut cur = self;

        loop {
            let temp = cur;
            cur = match temp.inner {
                Some(ref mut node) => &mut node.left,
                None               => return temp,
            }
        }

        cur
    }

    pub fn has(&mut self, key: K) -> bool {
        match *self.find_mut(&key, FindStrategy::Any) {
            Tree::Node(_) => true,
            Tree::Empty => false
        }
    }

    pub fn insert(&mut self, key: K) {
        let tree = self.find_mut(&key, FindStrategy::After);

        match *tree {
            Tree::Node(_) => unreachable!(),
            Tree::Empty => *tree = Tree::leaf(key)
        }
    }

    pub fn delete(&mut self, key: K) {
        let tree = self.find_mut(&key, FindStrategy::Any);
        
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
