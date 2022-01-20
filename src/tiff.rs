pub mod stream;
pub mod parser;
pub mod data_type;

pub use stream::*;
pub use parser::*;
pub use data_type::*;

#[derive(Clone, Debug)]
pub enum Compression {
  NoCompression,
  Jpeg,
  Unknown(u16)
}

#[derive(Clone, Debug)]
pub enum Entry {
  // See p.117
  // https://www.adobe.io/content/dam/udp/en/open/standards/tiff/TIFF6.pdf
  NewSubFileType {
    is_thumbnail: bool,
  },
  ImageWidth(u32),
  ImageLength(u32),
  BitsPerSample,
  Compression(Compression),
  PhotometricInterpretation,
  ImageDescription(String),
  Make(String),
  Model(String),
  Software(String),
  DateTime(String),
  // Unknown
  Unknown(u16, DataType, u32, u32)
}

#[derive(Clone, Debug)]
pub struct ImageFileDirectory {
  entries: Vec<Entry>,
}

#[derive(Clone, Debug)]
pub struct Tiff {
  directories: Vec<ImageFileDirectory>,
}
