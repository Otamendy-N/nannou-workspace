use crate::lib::linked_list::{queue::Queue, stack::Stack};

use super::lib::linked_list::LinkedList;

#[test]
fn queue() {
    let mut list: Box<dyn Queue<u32>> = Box::new(LinkedList::new());

    list.append(4);
    list.append(9);
    list.append(7);
    list.append(4);

    assert_eq!(list.size(), 4);
    assert_eq!(list.pop(), Some(4));
    assert_eq!(list.size(), 3);

    list.append(2);
    assert_eq!(list.pop(), Some(9));
    assert_eq!(list.pop(), Some(7));
    assert_eq!(list.pop(), Some(4));
    assert_eq!(list.size(), 1);

    list.pop();
    list.pop();
    assert_eq!(list.pop(), None);
    assert_eq!(list.size(), 0);
}

#[test]
fn stack() {
    let mut stack: Box<dyn Stack<u32>> = Box::new(LinkedList::new());

    stack.prepend(9);
    stack.prepend(8);
    stack.prepend(7);

    assert_eq!(stack.size(), 3);
    assert_eq!(stack.pop(), Some(7));
    assert_eq!(stack.pop(), Some(8));
    assert_eq!(stack.size(), 1);

    stack.pop();

    assert_eq!(stack.size(), 0);
    assert_eq!(stack.pop(), None)
}

#[test]
fn list() {
    let mut list = LinkedList::new();

    list.append("hola".to_string());
    list.append("mundo".to_string());
    list.append(":D".to_string());

    assert_eq!(list.size(), 3);
    assert_eq!(list.find_as_reference(|n| { n == "hola"}), Some(&"hola".to_string()));
    assert_eq!(list.size(), 3);
    let seven = list.find_as_mut_reference(|n| { n == "mundo"});
    let seven = seven.unwrap();
    seven.clear();
    assert_eq!(list.find(|x| { x == ""}), Some("".to_string()));
    assert_eq!(list.size(), 2);
}