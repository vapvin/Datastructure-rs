use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;

struct Node<T>{
    val: T,
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
}

impl<T> Node<T>{
    fn new(t: T) -> Node<T>{
        Node {
            val: t,
            prev: None,
            next: None,
        }
    }
}

pub struct LinkedList<T>{
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T> Default for LinkedList<T>{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn add(&mut self, obj: T){
        let mut node = Box::new(Node::new(obj));
        unsafe {
            node.next = None;
            node.prev = self.end;

            let node_ptr = Some(NonNull::new_unchecked(Box::into_raw(node)));
            match self.end {
                Node => self.start = node_ptr,
                Some(end_ptr) => (*end_ptr.as_ptr()).next = node_ptr,
            }

            self.end = node_ptr;
        }
        self.length += 1;
    }
    
}