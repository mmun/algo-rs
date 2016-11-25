#[macro_use] extern crate dmoj;
extern crate rand;

use std::fmt::Debug;
use std::cmp::Ordering::*;

use self::Dir::*;

#[derive(Clone,Copy)]
pub enum Dir { Left, Right }

impl Dir {
    fn flip(self) -> Self {
        if let Right = self { Left } else { Right }
    }

    fn smaller<T: Ord>(left: &T, right: &T) -> Self {
        if left < right { Left } else { Right }
    }
}

#[derive(Debug)]
pub struct Node<K> {
    key: K,
    priority: u32,
    size: u32,
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
            size: 0,
            left: Treap::new(),
            right: Treap::new()
        };

        Treap { inner: Some(Box::new(node)) }
    }

    pub fn size(&self) -> u32 {
        if self.is_node() {
            1 + self.node().size
        } else {
            0
        }
    }

    pub fn select(&mut self, index: u32) -> &K {
        let i = self.index();
        match index.cmp(&i) {
            Equal   => { println!("Equal: {:?}", self); flush!(); self.key() },
            Less    => { println!("Less: {:?}", self); flush!(); self.left().select(index) },
            Greater => { println!("Greater: {:#?}", self); flush!(); self.left().select(index - 1 - i) },
        }
    }

    pub fn order(&mut self, key: &K) -> Option<u32> {
        if self.is_node() {
            match key.cmp(self.key()) {
                Equal   => Some(self.left().size()),
                Less    => self.left().order(key),
                Greater => self.right().order(key).map(|order| 1 + self.left().size() + order)
            }
        } else {
            None
        }
    }

    pub fn insert(&mut self, key: K) {
        if self.is_node() {
            let dir = Dir::smaller(&key, self.key());

            self.node_mut().size += 1;
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

    pub fn index(&self) -> u32 {
        self.inner.as_ref().map_or(0, |ref n| n.left.size())
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
    fn inner(&self)         -> &Node<K>     { self.inner.as_ref().unwrap() }
    fn key(&self)           -> &K           { &self.node().key }
    fn priority(&self)      -> u32          { self.node().priority }    
    fn is_node(&self)       -> bool         { self.inner.is_some() }
    fn is_empty(&self)      -> bool         { self.inner.is_none() }
}


fn main() {
    let (n, m) = scan!(usize, usize);

    // let mut t = Treap::new();
    let mut t = Treap::new();
    for _ in 0..n { t.insert(scan!(i32)) }

    let mut ans = 0;
    for _ in 0..m {
        scan!(char);
        let op = scan!(char);
        let arg = scan!(i32);// ^ ans;

        match op {
            'I' => t.insert(arg),
            'R' => t.delete(&arg),
            'S' => {
                ans = *t.select(arg as u32);
                println!("{}", ans);
            },
            'L' => {
                ans = match t.order(&arg) {
                    Some(i) => i as i32 + 1,
                    None => -1
                };
                println!("{}", ans);
            },
            _ => unreachable!()
        }
    }
}
