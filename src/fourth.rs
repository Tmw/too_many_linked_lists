use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>,
}
impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None, tail: None }
    }

    pub fn push_front(&mut self, elem: T) {
        let new_head = Node::new(elem);
        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_head);
                self.head = Some(new_head);
            },

            None => {
                self.head = Some(new_head.clone());
                self.tail = Some(new_head);
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    new_head.borrow_mut().prev.take();
                    self.head = Some(new_head);
                },
                None => {
                    // there's no more next.
                    // list's tail must be self, let's take.
                    self.tail.take();
                }
            }

            Rc::try_unwrap(old_head)
                .ok()
                .unwrap()
                .into_inner()
                .elem
        })
    }
}

impl<T> Node<T> {
    pub fn new(elem: T) -> Rc<RefCell<Self>> {
        Rc::new(
            RefCell::new(
                Node {
                    elem,
                    prev: None,
                    next: None,
                }
            )
        )
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        // Can we verify this with a ton of items?
        while self.pop_front().is_some() {}
    }
}

#[cfg(test)]
mod tests {
    use super::List;

    #[test]
    fn test_basics() {
        let mut list = List::new();

        // empty list is empty
        assert_eq!(list.pop_front(), None);

        list.push_front(12);
        list.push_front(42);
        list.push_front(8);

        assert_eq!(list.pop_front(), Some(8));
        assert_eq!(list.pop_front(), Some(42));

        list.push_front(28);
        list.push_front(45);

        assert_eq!(list.pop_front(), Some(45));
        assert_eq!(list.pop_front(), Some(28));

        // check exhaustion
        assert_eq!(list.pop_front(), Some(12));
        assert_eq!(list.pop_front(), None);
    }

    use std::rc::{Rc, Weak};

    #[test]
    fn test_drop_does_release_rcs() {
        let mut list = List::new();

        let item_one = Rc::new(1);
        let item_two = Rc::new(2);
        let item_three = Rc::new(3);

        // take weak references to items
        let weak_item_one = Rc::downgrade(&item_one);
        let weak_item_two = Rc::downgrade(&item_two);
        let weak_item_three = Rc::downgrade(&item_three);

        // setup small chain of items
        list.push_front(item_one);
        list.push_front(item_two);
        list.push_front(item_three);

        // initiate dropping the list
        drop(list);

        // and verify we have no cycles anymore :)
        assert_eq!(Weak::strong_count(&weak_item_one), 0);
        assert_eq!(Weak::strong_count(&weak_item_two), 0);
        assert_eq!(Weak::strong_count(&weak_item_three), 0);
    }
}
