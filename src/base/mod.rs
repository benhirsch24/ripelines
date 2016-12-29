use std::collections::HashMap;
use std::rc::{Rc};
use std::cell::{RefCell, RefMut};

pub struct Buffer {
    size: usize,
    data: Vec<u8>
}

#[derive(Copy, Clone)]
pub struct EdgeIdx {
    pub idx: usize
}

pub struct Edge {
    name: String,
    buffers: Vec<Buffer>,
    source: NodeIdx,
    sink: NodeIdx
}

impl Edge {
    pub fn new(name: &str, src: NodeIdx, sink: NodeIdx) -> Edge {
        Edge {
            name: name.to_string(),
            buffers: vec!(),
            source: src,
            sink: sink
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
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

pub enum FilterState {
    CREATED,
    INITIALIZED,
    PAUSED,
    PLAYING
}

pub struct Element {
    pub name: String,
    pub state: FilterState
}

impl Element {
    pub fn new(name: &str) -> Element {
        Element {
            name: name.to_string(),
            state: FilterState::CREATED
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
    fn get_element(&self) -> &Element;
    fn get_mut_element(&mut self) -> &mut Element;

    fn set_filter_state(&mut self, state: FilterState);

    fn run(&mut self);
}

#[derive(Copy, Clone)]
pub struct NodeIdx {
    pub idx: usize
}

pub struct Node {
    name: String,
    filter: Rc<RefCell<Filter>>,
    pub incoming_edges: Vec<EdgeIdx>,
    pub outgoing_edges: Vec<EdgeIdx>
}

impl Node {
    pub fn new(filter: Rc<RefCell<Filter>>) -> Node {
        let name = filter.borrow().get_element().name.clone();
        Node {
            name: name,
            filter: filter,
            incoming_edges: vec!(),
            outgoing_edges: vec!()
        }
    }

    pub fn get_filter(&self) -> Rc<RefCell<Filter>> {
        self.filter.clone()
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}

pub struct Pipeline {
    running: bool,
    nodes: Vec<Node>,
    edges: Vec<Rc<RefCell<Edge>>>
}

impl Pipeline {
    pub fn new() -> Pipeline {
        Pipeline { running: false, nodes: vec!(), edges: vec!() }
    }

    pub fn add(&mut self, f: Rc<RefCell<Filter>>) -> Result<NodeIdx, String> {
        let node = Node::new(f);
        let idx = self.nodes.len();
        self.nodes.push(node);

        Ok(NodeIdx{ idx: idx })
    }

    pub fn get_node(&self, idx: NodeIdx) -> &Node {
        &self.nodes[idx.idx]
    }

    pub fn get_node_mut(&mut self, idx: NodeIdx) -> &mut Node {
        &mut self.nodes[idx.idx]
    }

    pub fn connect(&mut self, src: NodeIdx, sink: NodeIdx) -> Result<EdgeIdx, String> {
        // form a name so we can debug and stuff
        let edge_name = {
            let nodeA = self.get_node(src);
            let nodeA_name = nodeA.get_name();
            let nodeB = self.get_node(sink);
            let nodeB_name = nodeB.get_name();

            format!("{}-{}", nodeA_name, nodeB_name)
        };

        // give the edge idx to the nodes themselves
        let idx = self.edges.len();
        let edge_idx = EdgeIdx{ idx: idx };

        // push the edge on
        let edge = Rc::new(RefCell::new(Edge::new(&edge_name, src, sink)));
        self.edges.push(edge);

        {
            let mut nodeA = self.get_node_mut(src);
            nodeA.outgoing_edges.push(edge_idx);
        }

        {
            let mut nodeB = self.get_node_mut(sink);
            nodeB.incoming_edges.push(edge_idx);
        }

        Ok(edge_idx)
    }

    pub fn make_schedule(&mut self) -> Vec<usize> {
        let mut nodes: Vec<usize> = (0..self.nodes.len()).collect();
        let mut top_of_tree = vec!();

        for (idx, node) in self.nodes.iter().enumerate() {
            println!("{}", node.get_name());

            if node.incoming_edges.is_empty() {
                nodes.remove(idx);
                top_of_tree.push(idx);
            }
        }

        for node_idx in top_of_tree {
            let node = &self.nodes[node_idx];
            println!("Top of tree: {}", node.get_name());
            println!("Outgoing edges:");
            for edge_idx in &node.outgoing_edges {
                let edge = &self.edges[edge_idx.idx];
                println!("   {}", edge.borrow().get_name());
            }
        }

        nodes
    }

    pub fn run(&mut self) {
        if self.running {
            println!("Pipeline is already running");
            return;
        }

        let schedule = self.make_schedule();
    }
}
