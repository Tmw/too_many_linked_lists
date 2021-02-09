#[derive(Debug)]
pub struct Node<T> {
    elem: T,
    next: Link<T>
}

type Link<T> = Option<Box<Node<T>>>;

pub struct List<T> {
    head: Link<T>
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self {
            head: None
        }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Node {
            elem,
            next: self.head.take()
        };

        self.head = Some(Box::new(new_node));
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
                self.head = node.next;
                node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.elem
        })
    }
}

/// Drop implementation as to not blow the stack
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}
/// Iterator implemention using IntoIter wrapper struct
pub struct IntoIter<T>(List<T>);
impl<T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

/// Iterator implementation from scratch
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>
}
impl<T> List<T> {
    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_deref()
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

/// Mutable Iterator implementationfrom scratch
pub struct IterMut<'a, T>{
    next: Option<&'a mut Node<T>>
}

impl<T> List<T> {
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut { 
            next: self.head.as_deref_mut()
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T>{
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}


#[cfg(test)]
mod tests {
    use super::List;

    #[test]
    fn push_then_pop_returns_same_value() {
        let mut list = List::new();
        list.push(12);
        assert_eq!(list.pop(), Some(12));
    }

    #[test]
    fn drop_large_list_doesnt_blow_the_stack() {
        let mut list = List::new();
        for _i in 0..1_000_000 {
            list.push(12)
        }

        // drop the list without blowing the stack
        std::mem::drop(list);
    }

    #[test]
    fn test_peek() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);

        list.push(12);
        list.push(13);
        assert_eq!(list.peek(), Some(&13));
    }

    #[test]
    fn test_peek_mut(){
        let mut list = List::new();
        assert_eq!(list.peek_mut(), None);

        list.push(12);
        list.push(13);
        assert_eq!(list.peek_mut(), Some(&mut 13));

        list.peek_mut().map(|value| {
            *value = 42;
        });

        assert_eq!(list.peek(), Some(&42));
    }

    #[test]
    fn test_into_iter() {
        let mut list = List::new();
        list.push(12);
        list.push(42);
        list.push(32);


        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(32));
        assert_eq!(iter.next(), Some(42));
        assert_eq!(iter.next(), Some(12));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iter() {
        let mut list = List::new();
        list.push(12);
        list.push(42);
        list.push(32);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&32));
        assert_eq!(iter.next(), Some(&42));
        assert_eq!(iter.next(), Some(&12));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iter_mut(){
        let mut list = List::new();
        list.push(12);
        list.push(42);
        list.push(32);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 32));
        assert_eq!(iter.next(), Some(&mut 42));
        assert_eq!(iter.next(), Some(&mut 12));
        assert_eq!(iter.next(), None);
    }
}


