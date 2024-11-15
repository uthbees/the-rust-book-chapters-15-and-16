use std::mem;

struct Node {
    value: i32,
    next: Link,
}

enum Link {
    None,
    Exists(Box<Node>),
}

pub struct LinkedList {
    head: Link,
}

impl LinkedList {
    pub fn new() -> Self {
        LinkedList { head: Link::None }
    }

    pub fn push(&mut self, value: i32) {
        let new_head = Link::Exists(Box::new(Node {
            value,
            // mem::replace isn't ideal(?)
            next: mem::replace(&mut self.head, Link::None),
        }));
        self.head = new_head;
    }

    pub fn pop(&mut self) -> Option<i32> {
        // mem::replace isn't ideal(?)
        match mem::replace(&mut self.head, Link::None) {
            Link::None => None,
            Link::Exists(node) => {
                self.head = node.next;
                Some(node.value)
            }
        }
    }
}

// We manually implement drop since by default, it would do it recursively which would not be good
// for the call stack.
impl Drop for LinkedList {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::None);
        while let Link::Exists(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::None);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basics() {
        let mut list = LinkedList::new();

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
}
