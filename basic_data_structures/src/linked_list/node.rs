pub type Link<T> = Option<Box<Node<T>>>;

#[derive(Clone)]
pub struct Node<T> {
    pub data: Box<T>,
    pub next: Link<T>,
}

impl<T: Clone> Node<T> {
    pub fn new(value: T) -> Node<T> {
        Node {
            data: Box::new(value),
            next: None,
        }
    }
}