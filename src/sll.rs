use super::node::Node;

#[derive(Debug)]
pub struct SLL {
    pub head: Option<Box<Node>>,
    pub size: u32,
}

impl Default for SLL {
    fn default() -> Self {
        SLL {
            head: None,
            size: 0
        }
    }
}

impl SLL {
    pub fn is_empty(&self) -> bool {
        match &self.head {
            None => true,
            Some(_) => false
        }
    }

    pub fn add_last(&mut self, val: i64) {
        if self.is_empty() {
            self.head = Some(Box::new(Node {
                value: Some(val),
                next: None
            }));
        }
        else {
            let mut it = &mut self.head;
            while it.as_ref().unwrap().as_ref().next.as_ref().is_some() {
                it = &mut it.as_mut().unwrap().as_mut().next;
            }
            let new_node = Some(Box::new(Node {
                value: Some(val),
                next: None
            }));
            it.as_mut().unwrap().as_mut().next = new_node;
        }
        self.size += 1;
    }

    pub fn print_out(&self) {
        let mut it = self.head.as_ref();

        while it.is_some() {
            print!("{} ", it.unwrap().value.expect("err"));
            it = it.unwrap().next.as_ref();
        }
    }
}