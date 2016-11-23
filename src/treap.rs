use std::fmt::Debug;
use std::cmp::Ordering::*;
use rand;

use self::Dir::*;

#[derive(Clone,Copy)]
pub enum Dir { Left, Right }

impl Dir {
    fn flip(self) -> Self {
        match self {
            Left => Right,
            Right => Left
        }
    }

    fn smaller<T: Ord>(left: &T, right: &T) -> Self {
        if left < right {
            Left
        } else {
            Right
        }
    }
}

#[derive(Debug)]
pub struct Node<K> {
    key: K,
    priority: u32,
    left: Treap<K>,
    right: Treap<K>,
}

#[derive(Debug)]
pub struct Treap<K> {
    inner: Option<Box<Node<K>>>
}

impl<K: Ord+Debug> Treap<K> {
    pub fn new() -> Treap<K> {
        Treap { inner: None }
    }

    pub fn leaf(key: K) -> Treap<K> {
        let node = Node {
            key: key,
            priority: rand::random(),
            left: Treap::new(),
            right: Treap::new()
        };

        Treap { inner: Some(Box::new(node)) }
    }

    pub fn has(&mut self, key: &K) -> bool {
        if self.is_node() {
            match key.cmp(self.key()) {
                Less    => self.left().has(key),
                Greater => self.right().has(key),
                Equal   => true
            }
        } else {
            false
        }
    }

    pub fn insert(&mut self, key: K) {
        if self.is_node() {
            let dir = Dir::smaller(&key, self.key());

            self.child(dir).insert(key);
           
            if self.child(dir).priority() > self.priority() {
                self.rotate(dir.flip());
            }
        } else {
            *self = Treap::leaf(key);
        }
    }

    pub fn delete(&mut self, key: &K) {
        if self.is_node() {
            match key.cmp(self.key()) {
                Less    => self.left().delete(key),
                Greater => self.right().delete(key),
                Equal   => {
                    if self.left().is_empty() {
                        *self = self.right().take();
                    } else if self.right().is_empty() {
                        *self = self.left().take();
                    } else {
                        let dir = Dir::smaller(&self.child(Left).priority(), &self.child(Right).priority());
                        self.rotate(dir);
                        self.child(dir).delete(key);
                    }
                }
            }
        }
    }

    fn rotate(&mut self, dir: Dir) {
        let mut x = self.take();
        let mut y = x.child(dir.flip()).take();
        *x.child(dir.flip()) = y.child(dir).take();
        *y.child(dir) = x;
        *self = y;
    }

    fn child(&mut self, dir: Dir) -> &mut Self {
        match dir {
            Left => self.left(),
            Right => self.right()
        }   
    }

    // fn into_node(self)      -> Node<K>      { *self.inner.unwrap() }
    // fn into_key(self)       -> K            { self.into_node().key }

    fn take(&mut self)      -> Self         { Treap { inner: self.inner.take() } }
    fn node_mut(&mut self)  -> &mut Node<K> { self.inner.as_mut().unwrap() }
    fn left(&mut self)      -> &mut Self    { &mut self.node_mut().left }
    fn right(&mut self)     -> &mut Self    { &mut self.node_mut().right }
    fn node(&self)          -> &Node<K>     { self.inner.as_ref().unwrap() }
    fn key(&self)           -> &K           { &self.node().key }
    fn priority(&self)      -> u32          { self.node().priority }
    
    fn is_node(&self)       -> bool         { self.inner.is_some() }
    fn is_empty(&self)      -> bool         { self.inner.is_none() }
}

#[cfg(test)]
mod tests {
    use super::*;

    const F: bool = false;
    const T: bool = true;

    #[test]
    fn insert() {
        let mut bst = Treap::new();

        bst.insert(5);
        bst.insert(2);

        assert_eq!(
            (0..10).map(|i| bst.has(&i)).collect::<Vec<_>>(),
            [F, F, T, F, F, T, F, F, F, F]
        );

        bst.insert(3);
        bst.insert(1);
        bst.insert(7);

        assert_eq!(
            (0..10).map(|i| bst.has(&i)).collect::<Vec<_>>(),
            [F, T, T, T, F, T, F, T, F, F]
        );
    }

    #[test]
    fn insert_and_delete() {
        let mut bst = Treap::new();

        bst.insert(7);
        bst.insert(3);
        bst.insert(11);
        bst.insert(1);
        bst.insert(5);
        bst.insert(9);
        bst.insert(13);
        assert_eq!((0..15).map(|i| bst.has(&i)).collect::<Vec<_>>(), [F, T, F, T, F, T, F, T, F, T, F, T, F, T, F]);

        bst.insert(0);
        bst.insert(2);
        bst.insert(4);
        bst.insert(6);
        bst.insert(8);
        bst.insert(10);
        bst.insert(12);
        bst.insert(14);
        assert_eq!((0..15).map(|i| bst.has(&i)).collect::<Vec<_>>(), [T, T, T, T, T, T, T, T, T, T, T, T, T, T, T]);

        bst.delete(&7);
        assert_eq!((0..15).map(|i| bst.has(&i)).collect::<Vec<_>>(), [T, T, T, T, T, T, T, F, T, T, T, T, T, T, T]);
        bst.delete(&3);
        assert_eq!((0..15).map(|i| bst.has(&i)).collect::<Vec<_>>(), [T, T, T, F, T, T, T, F, T, T, T, T, T, T, T]);
        bst.delete(&11);
        assert_eq!((0..15).map(|i| bst.has(&i)).collect::<Vec<_>>(), [T, T, T, F, T, T, T, F, T, T, T, F, T, T, T]);
        bst.delete(&1);
        assert_eq!((0..15).map(|i| bst.has(&i)).collect::<Vec<_>>(), [T, F, T, F, T, T, T, F, T, T, T, F, T, T, T]);
        bst.delete(&5);
        assert_eq!((0..15).map(|i| bst.has(&i)).collect::<Vec<_>>(), [T, F, T, F, T, F, T, F, T, T, T, F, T, T, T]);
        bst.delete(&9);
        assert_eq!((0..15).map(|i| bst.has(&i)).collect::<Vec<_>>(), [T, F, T, F, T, F, T, F, T, F, T, F, T, T, T]);
        bst.delete(&13);
        assert_eq!((0..15).map(|i| bst.has(&i)).collect::<Vec<_>>(), [T, F, T, F, T, F, T, F, T, F, T, F, T, F, T]);
        bst.delete(&0);
        assert_eq!((0..15).map(|i| bst.has(&i)).collect::<Vec<_>>(), [F, F, T, F, T, F, T, F, T, F, T, F, T, F, T]);
        bst.delete(&2);
        assert_eq!((0..15).map(|i| bst.has(&i)).collect::<Vec<_>>(), [F, F, F, F, T, F, T, F, T, F, T, F, T, F, T]);
        bst.delete(&4);
        assert_eq!((0..15).map(|i| bst.has(&i)).collect::<Vec<_>>(), [F, F, F, F, F, F, T, F, T, F, T, F, T, F, T]);
        bst.delete(&6);
        assert_eq!((0..15).map(|i| bst.has(&i)).collect::<Vec<_>>(), [F, F, F, F, F, F, F, F, T, F, T, F, T, F, T]);
        bst.delete(&8);
        assert_eq!((0..15).map(|i| bst.has(&i)).collect::<Vec<_>>(), [F, F, F, F, F, F, F, F, F, F, T, F, T, F, T]);
        bst.delete(&10);
        assert_eq!((0..15).map(|i| bst.has(&i)).collect::<Vec<_>>(), [F, F, F, F, F, F, F, F, F, F, F, F, T, F, T]);
        bst.delete(&12);
        assert_eq!((0..15).map(|i| bst.has(&i)).collect::<Vec<_>>(), [F, F, F, F, F, F, F, F, F, F, F, F, F, F, T]);
        bst.delete(&14);
        assert_eq!((0..15).map(|i| bst.has(&i)).collect::<Vec<_>>(), [F, F, F, F, F, F, F, F, F, F, F, F, F, F, F]);
    }

    #[test]
    fn insert_and_delete_duplicates() {
        let mut bst = Treap::new();

        bst.insert(0);
        bst.insert(1);
        bst.insert(0);
        bst.insert(1);
        bst.insert(0);
        bst.insert(1);

        println!("{:#?}", bst);

        assert_eq!(
            (0..2).map(|i| bst.has(&i)).collect::<Vec<_>>(),
            [T, T]
        );

        bst.delete(&0);
        bst.delete(&0);
        bst.delete(&1);

        assert_eq!(
            (0..2).map(|i| bst.has(&i)).collect::<Vec<_>>(),
            [T, T]
        );

        bst.delete(&0);

        assert_eq!(
            (0..2).map(|i| bst.has(&i)).collect::<Vec<_>>(),
            [F, T]
        );
    }
}
