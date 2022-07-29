use std::fmt::Debug;

#[derive(Debug)]
pub struct Node {
    pub value: Option<i64>,
    pub next: Option<Box<Node>>
}

impl Default for Node {
    fn default() -> Self {
        Node {
            value: None,
            next: None
        }
    }
}