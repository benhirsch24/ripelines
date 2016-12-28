extern crate ripelines;
use std::rc::Rc;
use std::cell::RefCell;
use ripelines::base::*;
use ripelines::plugins::filesrc::*;
use ripelines::plugins::print::*;

fn main() {
    let mut pipeline = Pipeline::new();
    let mut filesrc = Filesrc::new("data/telltale.txt");
    let mut print = Print::new();

    {
        filesrc.set_chunk_size(1024);

        filesrc.connect(&mut print);
    }

    pipeline.add(Rc::new(RefCell::new(filesrc)));
    pipeline.add(Rc::new(RefCell::new(print)));

    pipeline.run();
}
