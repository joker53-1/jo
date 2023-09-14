use std::error::Error;
pub mod batchbitmap;
pub mod bitmap;

pub trait Flushable {
    fn flush(&mut self) -> Result<(), Box<dyn Error>>;
}
pub trait BatchBitmapI {
    fn set(&mut self, idx: usize, value: u64);
    fn get(&mut self, idx: usize) -> u64;
}

pub trait BitmapI: Flushable {
    fn set(&mut self, idx: usize, value: bool);
    fn get(&mut self, idx: usize) -> bool;
}