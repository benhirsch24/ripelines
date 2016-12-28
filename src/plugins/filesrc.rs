use base::*;
use std::cell::{RefCell, RefMut};

const DEF_CHUNK_SIZE: usize = 1024 * 1024;

pub struct Filesrc {
    base: Element,

    location: String,
    chunk_size: usize,
}

impl Filesrc {
    pub fn new(loc: &str) -> Filesrc {
        Filesrc{
            base: Element::new("filesrc"),
            location: loc.to_string(),
            chunk_size: DEF_CHUNK_SIZE
        }
    }

    pub fn set_chunk_size(&mut self, cs: usize) { self.chunk_size = cs; }
    pub fn get_chunk_size(&self) -> usize { self.chunk_size }
}

impl Filter for Filesrc {
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
