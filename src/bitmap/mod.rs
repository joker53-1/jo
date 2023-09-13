use std::error::Error;
pub mod batchbitmap;

pub trait Flushable {
    fn flush() -> Result<(), Box<dyn Error>>;
}
pub trait BatchBitmap: Flushable {
    fn set(idx: i64, value: i64);
    fn get(idx: i64) -> i64;
}

pub trait Bitmap: Flushable {
    fn set(idx: i64, value: bool);
    fn get(idx: i64) -> bool;
}