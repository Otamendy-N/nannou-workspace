pub type Link<T> = Option<Box<Node<T>>>;

#[derive(Clone)]
pub struct Node<T> {
    pub data: T,
    pub next: Link<T>,
}

impl<T: Copy> Node<T> {
    pub fn new(value: T) -> Node<T> {
        Node {
            data: value,
            next: None,
        }
    }
}