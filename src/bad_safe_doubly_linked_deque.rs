/*
 * Doubly linked list impletation in safe Rust using RefCells
 */
#![allow(dead_code)]

use std::cell::RefCell;
use std::rc::{Rc, Weak};

type NodeLink<T> = Option<Rc<RefCell<Node<T>>>>;
type NodeBackLink<T> = Option<Weak<RefCell<Node<T>>>>;

#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: NodeLink<T>,
    prev: NodeBackLink<T>,
}

impl<T> Node<T> {
    fn new(elem: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            elem,
            next: None,
            prev: None,
        }))
    }
}

#[derive(Debug)]
struct List<T> {
    head: NodeLink<T>,
    tail: NodeLink<T>,
}

impl<T> List<T> {
    fn new() -> Self {
        List {
            head: None,
            tail: None,
        }
    }

    fn push_front(&mut self, elem: T)
    where
        T: std::fmt::Debug,
    {
        let new_node = Node::new(elem);

        match self.head.take() {
            Some(old_head) => {
                // Non-empty list, tail stays in same position

                // 1. Old head points backwards to new node
                // old_head.prev should always be None, no need to keep track of it
                // TODO: implement using try_borrow_mut
                old_head.borrow_mut().prev = Some(Rc::downgrade(&new_node));

                // 2. New node points forward to old head
                new_node.borrow_mut().next = Some(old_head);

                // 3. Point head to new node
                self.head = Some(new_node);
            }
            None => {
                // Empty list case
                // Head and Tail should point to the new node

                self.tail = Some(new_node.clone());
                self.head = Some(new_node);
            }
        };
    }

    fn pop_front(&mut self) -> Option<T>
    where
        T: std::fmt::Debug,
    {
        match self.head.take() {
            None => None,
            Some(old_head) => {
                self.head = old_head.borrow().next.clone();

                Some(
                    Rc::try_unwrap(old_head)
                        .expect("Old head still referenced")
                        .into_inner()
                        .elem,
                )
            }
        }
    }

    fn push_back(&mut self, elem: T) {
        todo!("Push back not implemented")
    }

    fn pop_back(&mut self) -> Option<T> {
        todo!("Pop back not implemented")
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop_front(), None);

        // Populate list
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        // Check normal removal
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);

        list.push_front(1);
        // // Push some more just to make sure nothing's corrupted
        // list.push_front(4);
        // list.push_front(5);

        // // Check normal removal
        // assert_eq!(list.pop_front(), Some(5));
        // assert_eq!(list.pop_front(), Some(4));

        // // Check exhaustion
        // assert_eq!(list.pop_front(), Some(1));
        // assert_eq!(list.pop_front(), None);

        // // ---- back -----

        // // Check empty list behaves right
        // assert_eq!(list.pop_back(), None);

        // // Populate list
        // list.push_back(1);
        // list.push_back(2);
        // list.push_back(3);

        // // Check normal removal
        // assert_eq!(list.pop_back(), Some(3));
        // assert_eq!(list.pop_back(), Some(2));

        // // Push some more just to make sure nothing's corrupted
        // list.push_back(4);
        // list.push_back(5);

        // // Check normal removal
        // assert_eq!(list.pop_back(), Some(5));
        // assert_eq!(list.pop_back(), Some(4));

        // // Check exhaustion
        // assert_eq!(list.pop_back(), Some(1));
        // assert_eq!(list.pop_back(), None);
    }

    // #[test]
    // fn peek() {
    //     let mut list = List::new();
    //     assert!(list.peek_front().is_none());
    //     assert!(list.peek_back().is_none());
    //     assert!(list.peek_front_mut().is_none());
    //     assert!(list.peek_back_mut().is_none());

    //     list.push_front(1);
    //     list.push_front(2);
    //     list.push_front(3);

    //     assert_eq!(&*list.peek_front().unwrap(), &3);
    //     assert_eq!(&mut *list.peek_front_mut().unwrap(), &mut 3);
    //     assert_eq!(&*list.peek_back().unwrap(), &1);
    //     assert_eq!(&mut *list.peek_back_mut().unwrap(), &mut 1);
    // }

    // #[test]
    // fn into_iter() {
    //     let mut list = List::new();
    //     list.push_front(1);
    //     list.push_front(2);
    //     list.push_front(3);

    //     let mut iter = list.into_iter();
    //     assert_eq!(iter.next(), Some(3));
    //     assert_eq!(iter.next_back(), Some(1));
    //     assert_eq!(iter.next(), Some(2));
    //     assert_eq!(iter.next_back(), None);
    //     assert_eq!(iter.next(), None);
    // }
}
