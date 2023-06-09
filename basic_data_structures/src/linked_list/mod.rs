use std::fmt::{Display, Write};

// Importing the Node and Link types from the node module
pub mod node;
use node::Link;

/// A singly linked list that stores values of type `T`.
///
/// # Type parameters
///
/// * `T`: The type of values that the linked list stores.
///
/// # Examples
///
/// Creating a new linked list:
///
/// ```
/// use linkedlist::LinkedList;
///
/// let mut list = LinkedList::new();
/// assert_eq!(list.size(), 0);
/// ```
pub struct LinkedList<T> {
    pub head: Link<T>,
    pub size: usize,
}

impl<T: Clone + Display> Clone for LinkedList<T> {
    fn clone(&self) -> Self {
        Self { head: self.head.clone(), size: self.size.clone() }
    }
}

impl<T: Clone + Display> Display for LinkedList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut current = &self.head;
        let mut text = String::new();
        while let Some(node) = current {
            text = format!("{} -> {}", text, node.data);
            current = &node.next;
        }

        text.fmt(f)
    }
}

impl<T: Clone + Display> LinkedList<T> {
    /// Creates a new, empty linked list.
    ///
    /// # Returns
    ///
    /// A new `LinkedList` instance with no nodes.
    ///
    /// # Examples
    ///
    /// ```
    /// use linkedlist::LinkedList;
    ///
    /// let mut list = LinkedList::new();
    /// assert_eq!(list.size(), 0);
    /// ```
    pub fn new() -> LinkedList<T> {
        LinkedList {
            head: None,
            size: 0,
        }
    }

    pub fn for_each_mut<F>(&mut self, lamda: F)
        where F: Fn(&mut T, usize)
    {
        if self.head.is_none() { return; }
        let mut current = self.head.as_mut().unwrap();
        let mut cont: usize = 0;

        while current.next.is_some() {
            lamda(&mut current.data, cont);
            cont += 1;
            current = current.next.as_mut().unwrap()
        }
    }

    pub fn for_each<F>(&self, lamda: F)
        where F: Fn(&T, usize)
    {
        if self.head.is_none() { return; }
        let mut current = self.head.as_ref().unwrap();
        let mut cont: usize = 0;

        while current.next.is_some() {
            lamda(&current.data, cont);
            current = current.next.as_ref().unwrap();
            cont += 1;
        }
    }

    /// Removes and returns the last node in the linked list, if it exists.
    ///
    /// # Returns
    ///
    /// An `Option<T>` that contains the data of the removed node, or `None` if the linked list is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use linkedlist::LinkedList;
    ///
    /// let mut list = LinkedList::new();
    /// list.prepend(1);
    /// list.prepend(2);
    /// assert_eq!(list.remove(), Some(1));
    /// assert_eq!(list.size(), 1);
    /// ```
    pub fn remove(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        self.size -= 1;
        let mut current = &mut self.head;
        while current.as_ref().unwrap().next.is_some() {
            current = &mut current.as_mut().unwrap().next;
        }
        let last_node = current.take().unwrap();
        Some(*last_node.data)
    }

    /// Prints the contents of the linked list, starting from the head node.
    ///
    /// # Examples
    ///
    /// ```
    /// use linkedlist::LinkedList;
    ///
    /// let mut list = LinkedList::new();
    /// list.prepend(1);
    /// list.prepend(2);
    /// list.print();
    /// ```
    pub fn print(&self) {
        let mut current = &self.head;
        while let Some(node) = current {
            print!("{} -> ", node.data);
            current = &node.next;
        }
        print!("null\n\n");
    }

    /// Checks whether the linked list is empty.
    ///
    /// # Returns
    ///
    /// `true` if the linked list is empty, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use linkedlist::LinkedList;
    ///
    /// let mut list = LinkedList::new();
    /// assert!(list.is_empty());
    /// list.prepend(1);
    /// assert!(!list.is_empty());
    /// ````
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    /// Returns the value of the first node in the linked list, without removing it.
    ///
    /// # Returns
    ///
    /// * `Some(T)` - The value of the first node, if it exists.
    /// * `None` - If the linked list is empty.
    pub fn peek_head(&self) -> Option<T> {
        if let Some(node) = self.head.as_ref() {
            return Some(*node.data.clone());
        }
        return None;
    }

    /// Finds the first mutable reference to a node in the linked list that matches
    /// the given `match_expression`.
    ///
    /// # Arguments
    ///
    /// * `match_expression` - A closure that takes a node's value and returns `true`
    ///                        if the node should be matched.
    ///
    /// # Returns
    ///
    /// An optional mutable reference to the first matching node's value. Returns `None`
    /// if no matching node is found or if the linked list is empty.
    pub fn find_as_mut_reference<F>(&mut self, match_expression: F) -> Option<&mut T>
    where
        F: Fn(T) -> bool,
    {
        if self.head.is_none() {
            return None;
        }
        let mut current = &mut self.head;
        let mut wanted_node = None;
        while current.is_some() {
            let node_value = current.as_ref().unwrap().data.clone();

            if match_expression(*node_value) {
                wanted_node = Some(&mut current.as_mut().unwrap().data);
                break;
            }
            current = &mut current.as_mut().unwrap().next;
        }

        if let Some(value) = wanted_node {
            return Some(value);
        }

        None
    }

    /// Finds the first immutable reference to a node in the linked list that matches
    /// the given `match_expression`.
    ///
    /// # Arguments
    ///
    /// * `match_expression` - A closure that takes a node's value and returns `true`
    ///                        if the node should be matched.
    ///
    /// # Returns
    ///
    /// An optional immutable reference to the first matching node's value. Returns `None`
    /// if no matching node is found or if the linked list is empty.
    pub fn find_as_reference<F>(&mut self, match_expression: F) -> Option<&T>
    where
        F: Fn(T) -> bool,
    {
        if self.head.is_none() {
            return None;
        }
        let mut current = &mut self.head;
        let mut wanted_node = None;
        while current.is_some() {
            let node_value = current.as_ref().unwrap().data.clone();

            if match_expression(*node_value) {
                wanted_node = Some(&current.as_ref().unwrap().data);
                break;
            }
            current = &mut current.as_mut().unwrap().next;
        }

        if let Some(value) = wanted_node {
            return Some(value);
        }

        None
    }

    /// Searches the linked list for a node that matches the given predicate, and removes it if found.
    ///
    /// # Arguments
    ///
    /// * `match_expression` - A closure that accepts a value of type `T` and returns a `bool`. The closure is used as a predicate to search for a matching node in the linked list.
    ///
    /// # Returns
    ///
    /// * `Some(T)` - The value of the removed node, if a node was found and removed.
    /// * `None` - If no matching node was found in the linked list.
    pub fn find<F>(&mut self, match_expression: F) -> Option<T>
    where
        F: Fn(T) -> bool,
    {
        if self.head.is_none() {
            return None;
        }
        let mut current = &mut self.head;
        let mut wanted_node = None;
        while current.as_ref().unwrap().next.is_some() {
            let node_value = current
                .as_ref()
                .unwrap()
                .next
                .as_ref()
                .unwrap()
                .data
                .clone();

            if match_expression(*node_value) {
                wanted_node = current.as_mut().unwrap().next.take();
                break;
            }
            current = &mut current.as_mut().unwrap().next;
        }

        if let Some(node) = wanted_node {
            let previus = current.as_mut().unwrap();
            previus.next = node.next;
            self.size -= 1;
            return Some(*node.data);
        }

        None
    }

    /// Removes and returns the first node in the linked list.
    ///
    /// # Returns
    ///
    /// * `Some(T)` - The value of the removed node, if the linked list was not empty.
    /// * `None` - If the linked list was empty.
    pub fn pop(&mut self) -> Option<T> {
        if self.head.is_none() {
            return None;
        }
        let top_node = self.head.take().unwrap();
        self.head = top_node.next;
        self.size -= 1;
        return Some(*top_node.data);
    }

    /// Returns the number of nodes in the linked list.
    ///
    /// # Returns
    ///
    /// The number of nodes in the linked list, as an unsigned 32-bit integer.
    pub fn size(&self) -> u32 {
        self.size as u32
    }
}
