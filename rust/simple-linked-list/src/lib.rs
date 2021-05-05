use std::iter::FromIterator;

#[derive(Debug, Clone)]
pub struct SimpleLinkedList<T: Clone> {
    head: Option<Box<Node<T>>>,
    length: usize,
}
#[derive(Debug, Clone)]
struct Node<T: Clone> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T: Clone> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None, length: 0 }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn push(&mut self, data: T) {
        self.head = Some(Box::new(Node {
            data: data,
            next: self.head.take(),
        }));
        
        self.length += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().and_then(|head| {
            self.head = head.next;
            self.length -= 1;
            Some(head.data)
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|head| &head.data)
    }

    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut reverse_list = SimpleLinkedList::new();
        let mut list = self.clone();

        while list.peek().is_some() {
            reverse_list.push(list.pop().unwrap());
        }

        reverse_list
    }
}

impl<T: Clone> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut simple_linked_list = Self::new();

        for i in iter {
            simple_linked_list.push(i);
        }

        simple_linked_list
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T: Clone> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut v = Vec::new();
        let mut list = self.clone().rev();

        while list.peek().is_some() {
            v.push(list.pop().unwrap());
        }

        v
    }
}
