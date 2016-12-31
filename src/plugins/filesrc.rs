use base::*;
use std::fs::File;
use std::io::prelude::*;
use std::rc::Rc;
use std::cell::{RefCell};

const DEF_CHUNK_SIZE: usize = 1024 * 1024;

pub struct Filesrc {
    base: Element,
    state: FilterState,

    fd: Option<File>,
    location: String,
    chunk_size: usize,
}

impl Filesrc {
    pub fn new(loc: &str) -> Rc<RefCell<Filesrc>> {
        Rc::new(RefCell::new(Filesrc {
            base: Element::new("filesrc"),
            state: FilterState::CREATED,
            fd: None,
            location: loc.to_string(),
            chunk_size: DEF_CHUNK_SIZE
        }))
    }

    pub fn set_chunk_size(&mut self, cs: usize) { self.chunk_size = cs; }
    pub fn get_chunk_size(&self) -> usize { self.chunk_size }
}

impl Filter for Filesrc {
    fn get_element(&self) -> &Element { &self.base }
    fn get_mut_element(&mut self) -> &mut Element { &mut self.base }

    fn set_filter_state(&mut self, state: FilterState) {
        match state {
            FilterState::CREATED => {
                println!("Filesrc is created");
            },
            FilterState::INITIALIZED => {
                let fd = File::open(&self.location);
                match fd {
                    Ok(f) => {
                        self.fd = Some(f);
                        println!("Filesrc initialized!");
                    },
                    Err(e) => {
                        println!("Error initializing filesrc: {}", e)
                    }
                }
            },
            FilterState::PAUSED => {
                println!("Filesrc now paused");
            },
            FilterState::PLAYING => {
                println!("Filesrc now playing");
            },
            FilterState::DONE => {
                println!("Filesrc now done");
            }
        }
        self.state = state;
    }

    fn get_filter_state(&self) -> FilterState {
        self.state
    }

    fn run(&mut self, incoming_edges: Vec<Rc<RefCell<Edge>>>, outgoing_edges: Vec<Rc<RefCell<Edge>>>) -> bool {
        assert_eq!(incoming_edges.len(), 0);
        assert_eq!(outgoing_edges.len(), 1);

        let mut v = vec![0; self.chunk_size];
        let bytes_read_res = {
            let buf = &mut v;
            let fd = match self.fd.as_mut() {
                Some(f) => f,
                None => {
                    println!("Trying to run filesrc when it hasn't been initialized.");
                    self.state = FilterState::DONE;
                    return false
                }
            };

            fd.read(buf)
        };

        let bytes_read = match bytes_read_res {
            Ok(b) => b,
            Err(e) => {
                println!("Error reading or done: {}", e);
                self.state = FilterState::DONE;
                0
            }
        };

        if bytes_read == 0 {
            self.state = FilterState::DONE;
            return false
        }

        let buffer = Buffer::new(v, bytes_read);
        outgoing_edges[0].borrow_mut().push_buffer(buffer);

        true
    }
}
