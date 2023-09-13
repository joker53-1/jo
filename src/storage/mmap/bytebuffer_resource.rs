use memmap2::RemapOptions;

use crate::BaseResource;

use super::{bytebuffer::ByteBuffer, mapped_file::MappedFile, MappedResource, MmapT};

pub struct ByteBufferResource {
    mapped_file: MappedFile,
    pub(crate) content_buffer: ByteBuffer,
    pub len: usize,
}

impl ByteBufferResource {
    pub fn new(filepath: String, is_readonly: bool) -> Self {
        let init_len = 4 * 1024 * 1024;
        let mapped_file = MappedFile::new(filepath);
        let byte_buffer = mapped_file.map(0, init_len, is_readonly).unwrap();
        ByteBufferResource {
            mapped_file,
            content_buffer: byte_buffer,
            len: init_len,
        }
    }

    fn map_content_buffer(&mut self, min_size: usize) {
        if self.len < min_size {
            match &mut self.content_buffer.mmap_file {
                MmapT::Mmap(mm) => unsafe { mm.remap(min_size, RemapOptions::new()).unwrap() },
                MmapT::MmapMut(mmt) => unsafe { mmt.remap(min_size, RemapOptions::new()).unwrap() },
            }
            self.len = min_size;
        }
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
        let mut br = ByteBufferResource::new("/tmp/test".to_string(), true);
        br.map_content();
        let bf = br.content_buffer;
        assert_eq!(156, bf.get_byte(0));
        assert_eq!(6888, bf.get_u64(8));
    }

    #[test]
    fn test_byte_buffer_resource() {
        let mut br = ByteBufferResource::new("/tmp/test".to_string(), false);
        br.map_content();
        let mut bf = br.content_buffer;
        bf.put_byte(0, 156);
        bf.put_u64(8, 6888);
        assert_eq!(156, bf.get_byte(0));
        assert_eq!(6888, bf.get_u64(8));
    }
}
