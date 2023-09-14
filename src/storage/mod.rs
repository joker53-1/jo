
pub mod mmap;

pub use mmap::ByteBufferResource;

pub trait ByteOperat {
    fn get_u64(&self, position: usize) -> u64;
    fn put_u64(&mut self, position: usize, value: u64);
    fn get_byte(&self, position: usize) -> u8;
    fn put_byte(&mut self, position: usize, value: u8);
}

pub trait MappedResource {
    type BufferType;
    fn map(&self, position: u64, size: usize, is_readonly: bool) -> Result<Self::BufferType, ()>;
}

// file head list, usually store in a control file
pub enum HeadFields {
    CommonFields()
}

pub trait BaseResource{
    type BufferType;
    fn map_content(&mut self);
    fn resize(&mut self, size: usize);
    fn force(&self);
}

pub trait ByteBufferResource1:BaseResource {
    type HeadMeta;
    fn get_headmeta(head: HeadFields) -> Self::HeadMeta;
}

pub trait ByteBufferProvider {
    fn create<T>(&self, name: &str, is_readonly: bool) -> Box<dyn MappedResource<BufferType = T>>;
    fn create_folder(&self, relative_path: &str);
    fn list_files(&self, relative_path: &str);
    fn is_exist(&self, name: &str)->bool;
    fn delete(&self, name: &str);
    fn rename(&self, oldname: &str, newname: &str)-> bool;
}
