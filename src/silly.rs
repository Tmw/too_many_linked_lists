









pub struct Stack<T> {
    head: Link<T>
}

type Link<T> = Option<Box<Node<T>>>;

pub struct Node<T> {
    elem: T,
    next: Link<T>
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem,
            next: None
        });

        self.push_node(new_node);
    }

    fn push_node(&mut self, mut node: Box<Node<T>>) {
        node.next  = self.head.take();
        self.head = Some(node)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.pop_node().map(|node| {
            node.elem
        })
    }

    pub fn pop_node(&mut self) -> Option<Box<Node<T>>> {
        self.head.take().map(|mut node| {
            self.head = node.next.take();
            node
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

impl<T> Drop for Stack<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_value) = cur_link {
            cur_link = boxed_value.next.take();
        }
    }
}


pub struct List<T> {
    left: Stack<T>,
    right: Stack<T>
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            left: Stack::new(),
            right: Stack::new()
        }
    }

    pub fn push_left(&mut self, elem: T) {
        self.left.push(elem)
    }

    pub fn push_right(&mut self, elem: T) {
        self.right.push(elem)
    }

    pub fn pop_left(&mut self) -> Option<T> {
        self.left.pop()
    }

    pub fn pop_right(&mut self) -> Option<T> {
        self.right.pop()
    }

    pub fn peek_left(&self) -> Option<&T> {
        self.left.peek()
    }

    pub fn peek_right(&self) -> Option<&T> {
        self.right.peek()
    }

    pub fn go_left(&mut self) -> bool {
        self.left.pop_node().map(|node| {
            self.right.push_node(node)
        }).is_some()
    }

    pub fn go_right(&mut self) -> bool {
        self.right.pop_node().map(|node| {
            self.left.push_node(node)
        }).is_some()
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn walking() {
        let mut list = List::new();

        list.push_left(3);
        list.push_left(2);
        list.push_left(1);

        list.push_right(4);
        list.push_right(5);
        list.push_right(6);

        // assert peeking on either side
        assert_eq!(list.peek_left(), Some(&1));
        assert_eq!(list.peek_right(), Some(&6));

        // assert moving the list
        list.go_right();
        assert_eq!(list.peek_left(), Some(&6));
        assert_eq!(list.peek_right(), Some(&5));

        // move all the way to the right
        while list.go_right() {}
        assert_eq!(list.peek_left(), Some(&4));
        assert_eq!(list.peek_right(), None);

        while list.go_left() {}
        assert_eq!(list.peek_left(), None);
        assert_eq!(list.peek_right(), Some(&3));
    }
}





