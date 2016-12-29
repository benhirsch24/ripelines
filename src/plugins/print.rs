use base::*;
use std::rc::Rc;
use std::cell::{RefCell};

pub struct Print {
    base: Element
}

impl Print {
    pub fn new() -> Rc<RefCell<Print>> {
        Rc::new(RefCell::new(Print {
            base: Element::new("print")
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
            }
        }
    }

    fn run(&mut self) {
        print!("Filesrc ran");
    }
}
