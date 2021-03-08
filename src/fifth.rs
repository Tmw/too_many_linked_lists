use std::ptr;

pub struct List<T> {
    head: Link<T>,
    tail: *mut Node<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: ptr::null_mut(),
        }
    }

    pub fn push(&mut self, elem: T) {
        let mut new_tail = Box::new(Node {
            elem,
            next: None
        });

        let raw_tail: *mut _ = &mut *new_tail;
        if !self.tail.is_null() {
            unsafe {
                (*self.tail).next = Some(new_tail);
            }
        } else {
            self.head = Some(new_tail);
        }

        self.tail = raw_tail;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            let head = *head;
            self.head = head.next;

            if self.head.is_none() {
                self.tail = ptr::null_mut()
            }

            head.elem
        })
    }
}

#[cfg(test)]
mod tests {
    use super::List;

    #[test]
    fn test_basics() {
        let mut list = List::new();
        assert!(list.pop().is_none());

        list.push(12);
        list.push(13);
        list.push(14);

        assert_eq!(list.pop(), Some(12));
        assert_eq!(list.pop(), Some(13));

        list.push(4);
        list.push(5);

        assert_eq!(list.pop(), Some(14));
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(5));

        // exhaustive?
        assert_eq!(list.pop(), None);
    }
}
