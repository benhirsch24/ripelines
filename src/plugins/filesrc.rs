use base::*;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::rc::Rc;
use std::cell::{RefCell, Cell};

const DEF_CHUNK_SIZE: usize = 1024 * 1024;

pub struct Filesrc {
    base: Element,

    fd: Option<File>,
    location: String,
    chunk_size: usize,
}

impl Filesrc {
    pub fn new(loc: &str) -> Rc<RefCell<Filesrc>> {
        Rc::new(RefCell::new(Filesrc {
            base: Element::new("filesrc"),
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
                    Ok(f) => self.fd = Some(f),
                    Err(e) => println!("{}", e)
                }
            },
            FilterState::PAUSED => {
                println!("Filesrc now paused");
            },
            FilterState::PLAYING => {
                println!("Filesrc now playing");
            }
        }
    }

    fn run(&mut self) {
        let mut v = Vec::with_capacity(self.chunk_size);
        let buf = &mut v;
        let fd = match self.fd.as_mut() {
            Some(f) => f,
            None => { panic!("Running filesrc but WE HAVEN'T OPENED A FILE!!!"); }
        };

        fd.read_exact(buf);

        print!("Filesrc ran");
    }
}
