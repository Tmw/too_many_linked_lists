#![allow(dead_code)]
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct List<T> {
    head: Link<T>
}

#[derive(Debug)]
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

pub type Link<T> = Option<Arc<Node<T>>>;

#[derive(Debug)]
pub struct Node<T> {
    elem: T,
    next: Link<T>,
}

use std::fmt::Debug;
impl<T: Debug> List<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn append(&self, elem: T) -> List<T> {
        let new_node = Node {
            elem,
            next: self.head.clone()
        };

        List {
            head: Some(Arc::new(new_node))
        }
    }

    pub fn print_sequence(&self) {
        for node in self.iter() {
            println!("elem: {:?}", node);
        }
    }

    pub fn tail(&self) -> List<T> {
        List {
            head: self.head.as_ref().and_then( |node| node.next.clone() )
        }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map( |node| &node.elem )
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter { next: self.head.as_deref() }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(node) = head {
            if let Ok(mut node) = Arc::try_unwrap(node) {
                head = node.next.take();
            } else {
                break;
            }
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

#[cfg(test)]
mod tests {
    use super::List;

    #[test]
    fn test_basics() {
        let list = List::new();
        assert_eq!(list.head(), None);

        let list = list.append(2).append(3).append(4);
        assert_eq!(list.head(), Some(&4));

        let list = list.tail();
        assert_eq!(list.head(), Some(&3));

        let list = list.tail();
        assert_eq!(list.head(), Some(&2));

        let list = list.tail();
        assert_eq!(list.head(), None);

        // empty tail
        let list = list.tail();
        assert_eq!(list.head(), None);
    }

    #[test]
    fn test_iter() {
        let list = List::new().append(1).append(2).append(3);
        let mut iter = list.iter();

        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }

    #[test]
    fn test_multi_threaded() {
        let list = List::new().append(1).append(2);
        let list_a = list.clone();

        let handle = std::thread::spawn(move || {
            println!("Printing sequence list a");
            list_a.append(3).append(4).print_sequence();
        });


        handle.join().unwrap();
    }
}


