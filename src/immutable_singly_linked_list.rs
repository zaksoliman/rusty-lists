/*
 * The goal here is to build a list that can have multiple owners. We'll make heavy use of Rc/Arc to allow something that looks like this:
 * list1 -> A ---+
 *              |
 *              v
 * List2 ------> B -> C -> D
 *              ^
 *              |
 * List3 -> X ---+
 *
 * The node B will have multiple owners
*/
use std::rc::Rc;

#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Option<Rc<Node<T>>>,
}

#[derive(Debug, Default)]
pub struct List<T> {
    head: Option<Rc<Node<T>>>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn prepend(mut self, elem: T) -> Self {
        let tail = self.head;
        self.head = Some(Rc::new(Node { elem, next: tail }));
        self
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_deref().map(|n| &n.elem)
    }

    pub fn tail(self) -> Self {
        Self {
            head: self.head.map(|n| Rc::clone(&n)),
        }
    }

    pub fn iter(self) -> ListIntoIter {
        todo!("iter not implemented")
    }
}

struct ListIntoIter {}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let list = List::new();
        assert_eq!(list.head(), None);

        let list = list.prepend(1).prepend(2).prepend(3);
        assert_eq!(list.head(), Some(&3));

        let list = list.tail();
        assert_eq!(list.head(), Some(&2));

        let list = list.tail();
        assert_eq!(list.head(), Some(&1));

        let list = list.tail();
        assert_eq!(list.head(), None);

        // Make sure empty tail works
        let list = list.tail();
        assert_eq!(list.head(), None);
    }

    #[test]
    fn iter() {
        let list = List::new().prepend(1).prepend(2).prepend(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }
}
