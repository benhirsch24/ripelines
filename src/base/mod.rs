use std::collections::HashMap;
use std::rc::{Rc};
use std::cell::{RefCell, RefMut};
use std::borrow::BorrowMut;

pub struct Buffer {
    size: usize,
    data: Vec<u8>
}

pub struct Edge {
    name: String,
    buffers: Vec<Buffer>
}

impl Edge {
    pub fn new(name: &str) -> Edge {
        Edge {
            name: name.to_string(),
            buffers: vec!()
        }
    }

    pub fn push_buffer(&mut self, buffer: Buffer) -> Result<(), String> {
        self.buffers.push(buffer);

        Ok(())
    }

    pub fn pull_buffer(&mut self) -> Result<Buffer, String> {
        match self.buffers.pop() {
            Some(b) => Ok(b),
            None    => Err("List of buffers is empty".to_string())
        }
    }
}

pub struct Element {
    pub name: String,
    pub incoming_edges: HashMap<String, RefCell<Edge>>,
    pub outgoing_edges: HashMap<String, RefCell<Edge>>
}

impl Element {
    pub fn new(name: &str) -> Element {
        Element {
            name: name.to_string(),
            incoming_edges: HashMap::new(),
            outgoing_edges: HashMap::new()
        }
    }

    pub fn iterate_edges(edges: &HashMap<String, Edge>) {
    }
}

pub enum EdgeType {
    INCOMING,
    OUTGOING
}

pub trait Filter {
    fn get_element(&mut self) -> &mut Element;

    fn request_new_edge(&self, filter_type: EdgeType) -> Result<RefCell<Edge>, String>;

    fn connect(&mut self, other: &mut Filter) -> Result<(), String> {
        let my_edge = self.request_new_edge(EdgeType::OUTGOING)?;
        let other_edge = other.request_new_edge(EdgeType::INCOMING)?;

        let mut other_element = other.get_element();
        let mut my_element = self.get_element();

        my_element.outgoing_edges.insert(other_element.name.clone(), my_edge);
        other_element.incoming_edges.insert(my_element.name.clone(), other_edge);

        Ok(())
    }

    fn run(&mut self);
}

pub struct Pipeline {
    running: bool,
    filters: Vec<Rc<RefCell<Filter>>>
}

impl Pipeline {
    pub fn new() -> Pipeline {
        Pipeline {running: false, filters: vec!()}
    }

    pub fn add(&mut self, f: Rc<RefCell<Filter>>) {
        self.filters.push(f);
    }

    pub fn make_schedule(&mut self) -> Vec<usize> {
        vec!()
    }

    pub fn run(&mut self) {
        if self.running {
            println!("Pipeline is already running");
            return;
        }

        let schedule = self.make_schedule();
        println!("{}", schedule.len());
    }
}

