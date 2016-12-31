use std::collections::HashMap;
use std::collections::LinkedList;
use std::rc::{Rc};
use std::cell::{RefCell};

pub struct Buffer {
    pub size: usize,
    pub data: Vec<u8>
}

impl Buffer {
    pub fn new(data: Vec<u8>, size: usize) -> Buffer {
        Buffer {
            data: data,
            size: size
        }
    }
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

    pub fn push_buffer(&mut self, buffer: Buffer) {
        self.buffers.push(buffer);
    }

    pub fn pull_buffer(&mut self) -> Result<Buffer, String> {
        match self.buffers.pop() {
            Some(b) => Ok(b),
            None    => Err("List of buffers is empty".to_string())
        }
    }
}

#[derive(Copy, Clone)]
pub enum FilterState {
    CREATED,
    INITIALIZED,
    PAUSED,
    PLAYING,
    DONE
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
}

pub enum EdgeType {
    INCOMING,
    OUTGOING
}

pub trait Filter {
    fn get_element(&self) -> &Element;
    fn get_mut_element(&mut self) -> &mut Element;

    fn set_filter_state(&mut self, state: FilterState);
    fn get_filter_state(&self) -> FilterState;

    fn run(&mut self, incoming_edges: Vec<Rc<RefCell<Edge>>>, outgoing_edges: Vec<Rc<RefCell<Edge>>>) -> bool;
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
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

    pub fn get_edge(&self, idx: EdgeIdx) -> Rc<RefCell<Edge>> {
        self.edges[idx.idx].clone()
    }

    pub fn connect(&mut self, src: NodeIdx, sink: NodeIdx) -> Result<EdgeIdx, String> {
        // form a name so we can debug and stuff
        let edge_name = {
            let node_a = self.get_node(src);
            let node_a_name = node_a.get_name();
            let node_b = self.get_node(sink);
            let node_b_name = node_b.get_name();

            format!("{}-{}", node_a_name, node_b_name)
        };

        // give the edge idx to the nodes themselves
        let idx = self.edges.len();
        let edge_idx = EdgeIdx{ idx: idx };

        // push the edge on
        let edge = Rc::new(RefCell::new(Edge::new(&edge_name, src, sink)));
        self.edges.push(edge);

        {
            let mut node_a = self.get_node_mut(src);
            node_a.outgoing_edges.push(edge_idx);
        }

        {
            let mut node_b = self.get_node_mut(sink);
            node_b.incoming_edges.push(edge_idx);
        }

        Ok(edge_idx)
    }

    pub fn make_schedule(&mut self) -> Vec<NodeIdx> {
        let mut visited_nodes = HashMap::new();
        for i in 0..self.nodes.len() {
            visited_nodes.insert(NodeIdx{idx: i}, false);
        }

        let mut node_list: LinkedList<NodeIdx> = LinkedList::new();
        let mut node_sched: Vec<NodeIdx> = vec!();

        // get the nodes at the top of the tree ie who don't have incoming edges
        for (idx, node) in self.nodes.iter().enumerate() {
            println!("{}", node.get_name());

            if node.incoming_edges.is_empty() {
                node_list.push_back(NodeIdx{idx:idx});
                visited_nodes.insert(NodeIdx{idx:idx}, true);
            }
        }

        // visit each node
        while !node_list.is_empty() {
            let node_idx = match node_list.pop_front() {
                Some(n) => n,
                None => break
            };
            let node = self.get_node(node_idx);

            // if each incoming edge has been visited, add this idx to the schedule
            println!("\nVisiting {}", node.get_name());
            let mut colored = true;
            for edge_idx in &node.incoming_edges {
                let edge = &self.edges[edge_idx.idx];
                match visited_nodes.get(&edge.borrow().source) {
                    Some(c) => {
                        if !c {
                            colored = false;
                        }
                    },
                    None => {}
                }
            }

            if colored {
                println!("This node has been colored!!! {:?}", node_idx);
                node_sched.push(node_idx);
            }

            // add each node from the outgoing edges to the list
            println!("Outgoing edges:");
            for edge_idx in &node.outgoing_edges {
                let edge = &self.edges[edge_idx.idx];
                println!("   {}", edge.borrow().get_name());

                let edge_sink = edge.borrow().sink;
                let outgoing_node = self.get_node(edge_sink);
                match visited_nodes.get(&edge_sink) {
                    Some(c) => {
                        println!("   {} visited? {}", outgoing_node.get_name(), c);
                        if !c {
                            node_list.push_back(edge_sink);
                        }
                    },
                    None => { println!("   this shouldn't happen"); }
                }
            }
        }

        node_sched
    }

    pub fn run(&mut self) -> bool {
        if self.running {
            println!("Pipeline is already running");
            return false
        }

        let schedule = self.make_schedule();
        if schedule.is_empty() {
            return false
        }

        println!("\nSchedule is: ");
        for (idx, node_idx) in schedule.iter().enumerate() {
            let node = self.get_node(*node_idx);
            println!("{}: {}", idx, node.get_name());
        }

        println!();
        for node_idx in &schedule {
            let node = self.get_node(*node_idx);
            let filter = node.get_filter();
            let mut filter_mut = filter.borrow_mut();
            filter_mut.set_filter_state(FilterState::INITIALIZED);
        }

        println!();
        let mut idx = 0;
        loop {
            let node_idx = schedule[idx];
            let node = self.get_node(node_idx);
            {
                let filter = node.get_filter();
                let filter_b = filter.borrow();
                match filter_b.get_filter_state() {
                    FilterState::DONE => break,
                    _ => {}
                }
            }

            let mut incoming = vec!();
            let mut outgoing = vec!();
            for edge_idx in &node.incoming_edges {
                let edge = self.get_edge(*edge_idx).clone();
                incoming.push(edge);
            }
            for edge_idx in &node.outgoing_edges {
                let edge = self.get_edge(*edge_idx).clone();
                outgoing.push(edge);
            }

            {
                let filter = node.get_filter();
                let mut filter_m = filter.borrow_mut();
                if !filter_m.run(incoming, outgoing) {
                    break
                }
            }

            idx = (idx + 1) % schedule.len();
        }

        true
    }
}
