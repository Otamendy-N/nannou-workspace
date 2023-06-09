use crate::linked_list::{LinkedList, node::Node};


/// A generic trait for implementing a queue, which is a data structure that stores a collection
/// of elements in a first-in, first-out (FIFO) order.
pub trait Queue<T> {
    /// Appends the given `value` to the end of the queue.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to append to the queue.
    fn append(&mut self, value: T);
    
    /// Removes and returns the first element in the queue.
    ///
    /// # Returns
    ///
    /// An optional value representing the first element in the queue. Returns `None`
    /// if the queue is empty.
    fn pop(&mut self) -> Option<T>;
    
    /// Returns the number of elements currently in the queue.
    ///
    /// # Returns
    ///
    /// The number of elements in the queue as an unsigned 32-bit integer.
    fn size(&self) -> u32;
}


impl<T: Clone> Queue<T> for LinkedList<T> {
    fn append(&mut self, value: T) {
        self.size += 1;
        let new_node = Some(Box::new(Node::new(value)));
        if self.head.is_none() {
            self.head = new_node;
            return;
        }
        let mut current = &mut self.head;
        while current.as_ref().unwrap().next.is_some() {
            current = &mut current.as_mut().unwrap().next;
        }
        if let Some(node) = current {
            node.next = new_node;
        }
    }
    fn pop(&mut self) -> Option<T> {
        if self.head.is_none() {
            return None;
        }
        let top_node = self.head.take().unwrap();
        self.head = top_node.next;
        self.size -= 1;
        return Some(*top_node.data);
    }
    fn size(&self) -> u32 {
        self.size as u32
    }
}