use std::fmt::Debug;
use std::cmp::Ordering::*;

enum Find { Any, After }

#[derive(Debug)]
struct Node<K> {
    key: K,
    left: Tree<K>,
    right: Tree<K>,
}

#[derive(Debug)]
pub struct Tree<K> {
    inner: Option<Box<Node<K>>>
}

impl<K: Ord+Debug> Tree<K> {
   pub fn new() -> Tree<K> {
        Tree { inner: None }
    }

    pub fn leaf(key: K) -> Tree<K> {
        let node = Node {
            key: key,
            left: Tree::new(),
            right: Tree::new()
        };

        Tree { inner: Some(Box::new(node)) }
    }

    pub fn has(&mut self, key: K) -> bool {
        let tree = self.find_mut(&key, Find::Any);

        tree.is_node()
    }

    pub fn insert(&mut self, key: K) {
        let tree = self.find_mut(&key, Find::After);

        *tree = Tree::leaf(key);
    }

    pub fn delete(&mut self, key: K) {
        let tree = self.find_mut(&key, Find::Any); 

        if tree.is_node() {
            if tree.right_mut().is_node() {
                let succ = {
                    let succ = tree.right_mut().first_mut();
                    let mut taken = succ.take();
                    *succ = taken.right_mut().take();
                    taken
                };
                tree.node_mut().key = succ.into_key();
            } else {
                *tree = tree.left_mut().take();
            }
        }
    }

    fn find_mut(&mut self, key: &K, strategy: Find) -> &mut Self {
        let mut cur = self;

        loop {
            let temp = cur;
            if temp.is_node() {
                cur = match temp.key().cmp(key) {
                    Less    => temp.right_mut(),
                    Greater => temp.left_mut(),
                    Equal   => {
                        match strategy {
                            Find::After => temp.right_mut(),
                            Find::Any   => return temp
                        }
                    }
                };
            } else {
                return temp
            }
        }
    }

    fn first_mut(&mut self) -> &mut Self {
        if self.is_empty() { return self }

        let mut cur = self;
        loop {
            let temp = cur;
            if temp.left_mut().is_node() {
                cur = temp.left_mut()
            } else {
                return temp
            }
        }
    }

    fn into_node(self)      -> Node<K>      { *self.inner.unwrap() }
    fn into_key(self)       -> K            { self.into_node().key }

    fn take(&mut self)      -> Self         { Tree { inner: self.inner.take() } }
    fn node_mut(&mut self)  -> &mut Node<K> { self.inner.as_mut().unwrap() }
    fn left_mut(&mut self)  -> &mut Self    { &mut self.node_mut().left }
    fn right_mut(&mut self) -> &mut Self    { &mut self.node_mut().right }

    fn node(&self)          -> &Node<K>     { self.inner.as_ref().unwrap() }
    fn key(&self)           -> &K           { &self.node().key }
    fn is_node(&self)       -> bool         { self.inner.is_some() }
    fn is_empty(&self)      -> bool         { self.inner.is_none() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert() {
        let mut bst = Tree::new();

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
        let mut bst = Tree::new();

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
        let mut bst = Tree::new();

        bst.insert(0);
        bst.insert(1);
        bst.insert(0);
        bst.insert(1);
        bst.insert(0);
        bst.insert(1);

        println!("{:#?}", bst);

        assert_eq!(
            (0..2).map(|i| bst.has(i)).collect::<Vec<_>>(),
            [true, true]
        );

        bst.delete(0);
        println!("{:#?}", bst);
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
