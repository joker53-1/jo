
mod mmap;

pub trait ByteOperat {
    unsafe fn get_i64(&self, position: usize) -> i64;
    unsafe fn put_i64(self, position: usize, value: i64);
    unsafe fn get_byte(&self, position: usize) -> u8;
    unsafe fn put_byte(self, position: usize, value: u8);
}

pub trait MappedResource {
    type BufferType;
    fn map(&self, position: u64, size: usize, is_readonly: bool) -> Result<Self::BufferType, ()>;
}
