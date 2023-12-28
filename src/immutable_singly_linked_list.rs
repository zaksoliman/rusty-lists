/*
 * The goal here is to build a list that can have multiple owners. We'll make heavy use of Rc/Arc
 */

use std::rc::Rc;

struct Node<T> {
    elem: T,
    next: Option<Rc<Node<T>>>,
}

pub struct List<T> {
    head: Node<T>,
    tail: Node<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        todo!("Implement new()")
    }

    pub fn prepend(self) -> Self {
        todo!("prepend not implemented")
    }

    pub fn head(self) -> Self {
        todo!("head not implemented")
    }

    pub fn tail(self) -> Self {
        todo!("tail not implemented")
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
