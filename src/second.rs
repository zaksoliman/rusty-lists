/* We build on the first list implementation by adding the following features:
*   1. Making it Generic
*    2. Adding peek() feature to look at the value without taking it
*    3. Make list iterable
*/

#![allow(dead_code)]

#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug, Default)]
pub struct List<T> {
    head: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: Link::None }
    }

    pub fn push(&mut self, new: T) {
        let node_link = Link::Some(Box::new(Node {
            elem: new,
            next: self.head.take(),
        }));

        self.head = node_link;

        // mem::swap(&mut node_link, &mut self.head);
        // if let Link::Some(ref mut node) = self.head {
        //     node.next = node_link;
        // }
    }

    pub fn pop(&mut self) -> Option<T> {
        let popped = self.head.take();

        popped.map(|x| {
            self.head = x.next;
            x.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|s| &s.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|n| &mut n.elem)
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        /*
         * We are required to define a iterative drop strategy because it will blow the stack otherwise.
         * No tail call recursion because the box needs to drop it's pointers value before deallocating
         */
        let mut current_node = self.head.take();
        while let Some(mut boxed_node) = current_node {
            current_node = boxed_node.next.take();
            // boxed_node goes out of scope and gets dropped here;
            // but its Node's `next` field has been set to Link::Empty
            // so no unbounded recursion occurs.
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn no_stack_overflow_when_drop_big_list() {
        {
            let mut list = List::new();
            for i in 1..100000 {
                list.push(i);
            }
        }

        println!("Dropped list");
    }
    #[test]
    fn peek() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));

        list.peek_mut().map(|value| *value = 42);

        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.pop(), Some(42));
    }
}
