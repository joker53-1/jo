use std::fs::File;
use std::io::Read;

use memmap2::{Mmap, MmapOptions};

fn main() {
    let mut file = File::open("test").unwrap();

    let mut contents = Vec::new();
    file.read_to_end(&mut contents).unwrap();

    
    let mmap = unsafe { Mmap::map(&file).unwrap() };

    for i in &contents {
        println!("{}", i);
    }
    assert_eq!(&contents[..], &mmap[..]);
}
