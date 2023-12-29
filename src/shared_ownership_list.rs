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

struct Node<T> {
    elem: T,
    next: Option<Rc<Node<T>>>,
}

impl<T: std::fmt::Debug> std::fmt::Debug for Node<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let strong_count = match &self.next {
            Some(node) => Rc::strong_count(node),
            None => 1000,
        };
        write!(
            f,
            "elem: {:?}, next({:?}):{:?}",
            self.elem, strong_count, self.next
        )
    }
}

#[derive(Debug, Default)]
pub struct List<T> {
    head: Option<Rc<Node<T>>>,
}

impl<T: std::fmt::Debug> List<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    // Takes a list and an elem and returns new list.
    pub fn prepend(&self, elem: T) -> Self {
        println!("Prepend {:?}", elem);
        dbg!(self);

        let l = List {
            head: Some(Rc::new(Node {
                elem,
                next: self.head.clone(),
            })),
        };

        println!("\nDone");
        dbg!(&l);

        l
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_deref().map(|n| &n.elem)
    }

    pub fn tail(&self) -> Self {
        match &self.head {
            Some(node) => Self {
                head: node.next.as_ref().map(Rc::clone),
            },
            None => Self { head: None },
        }
    }

    pub fn iter(&self) -> ListIntoIter<T> {
        ListIntoIter {
            current: self.head.as_deref(),
        }
    }
}

pub struct ListIntoIter<'a, T> {
    current: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for ListIntoIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.map(|n| {
            self.current = n.next.as_deref();
            &n.elem
        })
    }
}

// impl<T> Drop for List<T> {
//     fn drop(&mut self) {
//         let mut current_node = self.head.take();
//     }
// }

#[cfg(test)]
mod test {
    use super::List;
    use std::rc::Rc;

    #[test]
    fn basics() {
        let list = List::new();
        assert_eq!(list.head(), None);

        let list = list.prepend(1).prepend(2).prepend(3);
        dbg!(&list);
        assert_eq!(list.head(), Some(&3));

        let list = list.tail();
        dbg!(&list);
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
    #[test]
    fn trident() {
        let list = List::new();

        let list1 = list.prepend(1).prepend(2).prepend(3).prepend(4);

        let list2 = list1.tail();
        let list3 = list2.prepend(5);

        assert_eq!(list1.head(), Some(&4));
        assert_eq!(list2.head(), Some(&3));
        assert_eq!(Rc::strong_count(&list2.head.unwrap()), 3);
        assert_eq!(list3.head(), Some(&5));
    }
    // #[test]
    // fn drop_large_list() {
    //     // New scope
    //     {
    //         let mut list = List::new();
    //         for i in 1..100000 {
    //             list = list.prepend(i);
    //         }
    //     }
    //     // Drop list
    //     println!("Dropped list");
    // }
}
