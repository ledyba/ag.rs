pub mod stream;
pub mod parser;
pub mod data_type;

use log::info;
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
pub enum Orientation { // Counter-clockwise
  Rotate0,
  Rotate180,
  Rotate270,
  Rotate90,
  Unknown,
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
  Orientation(Orientation),
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

impl Tiff {
  pub fn inspect(&self) {
    info!("** Tiff **");
    for (i, dir) in (0..).zip(self.directories.iter()) {
      self.inspect_dir(i, dir, 0);
    }
  }
  fn inspect_dir(&self, i: i32, dir: &ImageFileDirectory, indent: usize) {
    info!("{:indent$}<<ImageFileDirectory {}>>", " ", i);
    for (_, ent) in (0..).zip(dir.entries.iter()) {
      info!("{:indent$}- {:?}", " ", ent, indent = indent + 2);
    }
  }
}