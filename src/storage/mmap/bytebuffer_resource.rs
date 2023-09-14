use crate::BaseResource;

use super::{bytebuffer::ByteBuffer, mapped_file::MappedFile, MappedResource};

pub struct ByteBufferResource {
    pub(crate) mapped_file: MappedFile,
    pub(crate) content_buffer: Option<ByteBuffer>,
    pub len: usize,
    is_readonly: bool
}

impl ByteBufferResource {
    pub fn new(filepath: String, is_readonly: bool) -> Self {
        // let init_len = 4 * 1024 * 1024;
        let mapped_file = MappedFile::new(filepath);
        // let byte_buffer = mapped_file.map(0, init_len, is_readonly).unwrap();
        ByteBufferResource {
            mapped_file,
            content_buffer: None,
            len: 0,
            is_readonly
        }
    }

    fn map_content_buffer(&mut self, min_size: usize) {
        if self.len < min_size {

            self.content_buffer = Some(self.mapped_file.map(0, min_size, self.is_readonly).unwrap());
                // match &mut self.content_buffer.as_mut().unwrap().mmap_file {
                //     MmapT::Mmap(mm) => unsafe { mm.remap(min_size, RemapOptions::new()).unwrap() },
                //     MmapT::MmapMut(mmt) => unsafe { mmt.remap(min_size, RemapOptions::new()).unwrap() },
                // }
        
            self.len = min_size;
        }
    }

    pub fn fill(&mut self, pos: usize, size: usize, value: u8){
        self.content_buffer.as_mut().unwrap().fill(pos, size, value);
    }

}

impl BaseResource for ByteBufferResource {
    type BufferType = ByteBuffer;

    fn map_content(&mut self) {
        self.map_content_buffer(self.len);
    }

    fn resize(&mut self, size: usize) {
        self.map_content_buffer(size);
    }

    fn force(&self) {
        self.mapped_file.force();
    }
}

#[cfg(test)]
mod test {
    use crate::{storage::ByteOperat, BaseResource};

    use super::ByteBufferResource;

    #[test]
    fn test_readonly_byte_buffer_resource() {
        let mut br = ByteBufferResource::new("./test".to_string(), true);
        br.resize(7);
        let bf = br.content_buffer.unwrap();
        assert_eq!(156, bf.get_byte(0));
        assert_eq!(6888, bf.get_u64(8));
    }

    #[test]
    fn test_byte_buffer_resource() {
        let mut br = ByteBufferResource::new("./test".to_string(), false);
        br.resize(72);
        let mut bf = br.content_buffer.unwrap();
        bf.put_byte(0, 156);
        bf.put_u64(8, 6888);
        assert_eq!(156, bf.get_byte(0));
        assert_eq!(6888, bf.get_u64(8));
    }
}
