use std::fmt::Display;

// Importing the Node and Link types from the node module
mod node;
use node::{ Link, Node };

// Defining the List struct
pub struct List<T> {
    head: Link<T>,
    pub size: usize,
}

impl<T: Copy + Display> List<T> {
    // A constructor function that returns a new List instance with a None head and length of 0
    pub fn new() -> List<T> {
        List {
            head: None,
            size: 0,
        }
    }
    // A method that adds a new node to the end of the list with the given value
    pub fn push(&mut self, value: T) {
        self.size += 1; // Incrementing the length of the list
        let new_node = Some(Box::new(Node::new(value)));
        if self.is_empty() {
            self.head = new_node;
            return;
        }
        let mut current = &mut self.head;
        while current.as_ref().unwrap().next.is_some(){
            current = &mut current.as_mut().unwrap().next;
        }
        if let Some(node) = current {
            node.next = new_node;
        }
    }
    // A method that removes the last node from the list and returns its value as an Option<T>
    pub fn pop(&mut self) -> Option<T> {
        // If the list is empty, return None
        if self.is_empty() {
            return None;
        }
        self.size -= 1;
        let mut current = &mut self.head;
        // Traverse the list until the last node is reached (i.e., the node whose next pointer is None)
        while current.as_ref().unwrap().next.is_some() {
            current = &mut current.as_mut().unwrap().next;
        }
        // Take ownership of the last node and extract its data value, which is then returned wrapped in Some
        let last_node = current.take().unwrap();
        Some(last_node.data)
    }
    // A method that prints the contents of the list
    pub fn print(&self) {
        let mut current = &self.head;
        while let Some(node) = current {
            print!("{} -> ", node.data); // Print the data value of the current node
            current = &node.next; // Advance the current node to the next node in the list
        }
        print!("null\n\n"); // Print "null" to indicate the end of the list
    }
    // A method that checks whether the list is empty
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }
}
