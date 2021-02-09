#[derive(Debug)]
pub struct Node {
    elem: i32,
    next: Link
}

pub struct List {
    head: Link
}

#[derive(Debug)]
enum Link {
    Empty,
    More(Box<Node>)
}

impl List {
    pub fn new() -> Self {
        Self {
            head: Link::Empty
        }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Node {
            elem,
            next: std::mem::replace(&mut self.head, Link::Empty)
        };

        self.head = Link::More(Box::new(new_node));
    }

    pub fn pop(&mut self) -> Option<i32> {
        match std::mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        use std::mem;

        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
        }
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
}


