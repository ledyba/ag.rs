pub mod stream;
pub mod parser;
pub mod tag;
pub mod data_type;

pub use stream::*;
pub use parser::*;
pub use tag::*;
pub use data_type::*;

#[derive(Clone, Debug)]
pub enum Entry {
  Unknown(Tag, TypeTag, u32, u32)
}

#[derive(Clone, Debug)]
pub struct ImageFileDirectory {
  entries: Vec<Entry>,
}

#[derive(Clone, Debug)]
pub struct Tiff {
  directories: Vec<ImageFileDirectory>,
}
