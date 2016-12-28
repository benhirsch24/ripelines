use base::*;
use std::cell::{RefCell, RefMut};

pub struct Print {
    base: Element
}

impl Print {
    pub fn new() -> Print {
        Print {
            base: Element::new("print")
        }
    }
}

impl Filter for Print {
    fn get_element(&mut self) -> &mut Element { &mut self.base }

    fn request_new_edge(&self, filter_type: EdgeType) -> Result<RefCell<Edge>, String> {
        let edge;

        match filter_type {
            EdgeType::INCOMING => {
                if self.base.incoming_edges.len() == 0 {
                    edge = RefCell::new(Edge::new(&format!("{}-incoming", self.base.name)))
                } else {
                    return Err(format!("Incoming edge already exists for {}", self.base.name))
                }
            },
            EdgeType::OUTGOING => {
                if self.base.outgoing_edges.len() == 0 {
                    edge = RefCell::new(Edge::new(&format!("{}-outgoing", self.base.name)))
                } else {
                    return Err(format!("Outgoing edge already exists for {}", self.base.name))
                }
            }
        }

        Ok(edge)
    }

    fn run(&mut self) {
        print!("Filesrc ran");
    }
}
