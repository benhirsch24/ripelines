use base::*;
use std::rc::Rc;
use std::cell::{RefCell};
use std::str;

pub struct Print {
    base: Element,
    state: FilterState
}

impl Print {
    pub fn new() -> Rc<RefCell<Print>> {
        Rc::new(RefCell::new(Print {
            base: Element::new("print"),
            state: FilterState::CREATED
        }))
    }
}

impl Filter for Print {
    fn get_element(&self) -> &Element { &self.base }
    fn get_mut_element(&mut self) -> &mut Element { &mut self.base }

    fn set_filter_state(&mut self, state: FilterState) {
        match state {
            FilterState::CREATED => {
                println!("Print is created");
            },
            FilterState::INITIALIZED => {
                println!("Print now initialized");
            },
            FilterState::PAUSED => {
                println!("Print now paused");
            },
            FilterState::PLAYING => {
                println!("Print now playing");
            },
            FilterState::DONE => {
                println!("Print now done");
            }
        }

        self.state = state;
    }

    fn get_filter_state(&self) -> FilterState {
        self.state
    }

    fn run(&mut self, incoming_edges: Vec<Rc<RefCell<Edge>>>, outgoing_edges: Vec<Rc<RefCell<Edge>>>) -> bool {
        assert_eq!(incoming_edges.len(), 1);
        assert_eq!(outgoing_edges.len(), 0);

        let buffer = match incoming_edges[0].borrow_mut().pull_buffer() {
            Ok(buf) => buf,
            Err(e)  => {
                println!("Error pulling buffer for print: \"{}\"", e);
                self.state = FilterState::DONE;
                return false
            }
        };

        if buffer.size == 0 {
            self.state = FilterState::DONE;
            return false
        }

        match str::from_utf8(&buffer.data) {
            Ok(s) => print!("{}", s),
            Err(e) => println!("Error parsing buffer data: {}", e)
        };

        true
    }
}
