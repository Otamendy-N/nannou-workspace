use crate::linked_list::{LinkedList, node::Node};


/// A trait representing a stack data structure that can store values of type `T`.
pub trait Stack<T> {
    /// Adds a new value to the top of the stack.
    ///
    /// # Arguments
    ///
    /// * `value`: The value to add to the top of the stack.
    fn prepend(&mut self, value: T);

    /// Removes and returns the top value from the stack, if the stack is not empty.
    ///
    /// Returns `None` if the stack is empty.
    ///
    /// # Returns
    ///
    /// An `Option<T>` that contains the top value of the stack, or `None` if the stack is empty.
    fn pop(&mut self) -> Option<T>;

    /// Returns the number of values in the stack.
    ///
    /// # Returns
    ///
    /// The number of values currently in the stack.
    fn size(&self) -> u32;
}

impl<T: Clone> Stack<T> for LinkedList<T> {
    fn prepend(&mut self, value: T) {
        let mut new_node = Box::new(Node::new(value));
        new_node.next = self.head.take();
        self.head = Some(new_node);
        self.size += 1;
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