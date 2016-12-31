extern crate ripelines;
use ripelines::base::*;
use ripelines::plugins::filesrc::*;
use ripelines::plugins::print::*;

fn main() {
    let mut pipeline = Pipeline::new();
    let filesrc = Filesrc::new("/Users/hirschb/Personal/ripelines/data/telltale.txt");
    let print = Print::new();

    {
        let mut fs_mut = filesrc.borrow_mut();
        fs_mut.set_chunk_size(10);
    }

    let filesrc_idx = match pipeline.add(filesrc) {
        Ok(idx) => idx,
        Err(s)  => { println!("Error adding filesrc: {}", s); return; }
    };

    let print_idx = match pipeline.add(print) {
        Ok(idx) => idx,
        Err(s)  => { println!("Error adding print: {}", s); return; }
    };

    match pipeline.connect(filesrc_idx, print_idx) {
        Ok(idx) => idx,
        Err(s) => { println!("Error connecting filesrc to print: {}", s); return; }
    };

    if !pipeline.run() {
        println!("Failed to run pipeline :(");
    }
}
