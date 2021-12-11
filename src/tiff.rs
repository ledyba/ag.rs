pub mod stream;
pub mod parser;

use std::collections::HashMap;
pub use stream::{Stream, ByteOrder};
pub use parser::Parser;

pub enum Entry {

}

pub struct Tiff {
  entries: Vec<Entry>
}
