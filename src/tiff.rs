pub mod stream;
pub mod parser;
pub mod data_type;

pub use stream::*;
pub use parser::*;
pub use data_type::*;

#[derive(Clone, Debug)]
pub enum SubFileType {
  FullResolution = 1,
  ReducedResolution = 2,
  SinglePageOfMultiPage = 3,
}

#[derive(Clone, Debug)]
pub enum Entry {
  // See p.117
  // https://www.adobe.io/content/dam/udp/en/open/standards/tiff/TIFF6.pdf
  NewSubFileType,
  SubFileType,
  ImageWidth(u32),
  ImageLength(u32),
  BitsPerSample,
  Compression,
  PhotometricInterpretation,
  Thresholding,
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
