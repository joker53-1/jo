use std::{path::Path, fs::{File, OpenOptions}};

use memmap2::{Mmap, MmapOptions, MmapMut};

use super::{MappedResource, bytebuffer::ByteBuffer};


pub struct MappedFile {
    filepath: Path
}

pub enum MmapT {
    Mmap(Mmap),
    MmapMut(MmapMut)
}

impl MappedResource for MappedFile {
    type BufferType = ByteBuffer;

    fn map(&self, position: u64, size: usize, is_readonly: bool) -> Result<Self::BufferType, ()> {
        let mmap_file = if is_readonly {
            let file = File::open(&self.filepath).unwrap();
            let mmap = unsafe {
                MmapOptions::new().offset(position).len(size).map(&file).unwrap()
            }; 
            MmapT::Mmap(mmap)
        }else {
            let file = OpenOptions::new().read(true).write(true).open(&self.filepath).unwrap();
            let mmap = unsafe { MmapOptions::new().offset(position).len(size).map_mut(&file).unwrap() };
            MmapT::MmapMut(mmap)
        };
        Ok(ByteBuffer::new(mmap_file))
    }

}
