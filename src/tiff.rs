pub mod stream;
pub mod parser;
pub mod data_type;

use log::info;
pub use stream::*;
pub use parser::*;
pub use data_type::*;

#[derive(Clone, Debug)]
pub enum Compression {
  // (TIFF/EP p.30)
  // Data is packed as tightly as possible into bytes,
  // padding at the end of the row to assure that each row's data ends
  // upon a byte boundary.
  NoCompression,
  // (TIFF/EP p.30)
  // TIFF/EP readers are only required to support Baseline DCT JPEG method.
  BaselineJpeg,
  Unknown(u16)
}

#[derive(Clone, Debug)]
pub enum Orientation { // Counter-clockwise
  Rotate0,
  Rotate180,
  Rotate270,
  Rotate90,
  Undefined(u16),
}

#[derive(Clone, Debug)]
pub enum ResolutionUnit {
  Unknown,
  Inch,
  Centimeter,
  Undefined(u16),
}

#[derive(Clone, Debug)]
pub enum Entry {
  // [TIFF] p.117
  // [TIFF/EP] p.17
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
  XResolution(UnsignedRational),
  YResolution(UnsignedRational),
  ResolutionUnit(ResolutionUnit),
  Software(String),
  DateTime(String),
  WhitePoint{x: UnsignedRational, y: UnsignedRational},
  PrimaryChromaticities{
    red_x: UnsignedRational, red_y: UnsignedRational,
    green_x: UnsignedRational, green_y: UnsignedRational,
    blue_x: UnsignedRational, blue_y: UnsignedRational,
  },
  SubIFDs(Vec<ImageFileDirectory>),
  JPEGInterChangeFormat(bool),
  JPEGInterChangeFormatLength(u32),
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
    info!("{:indent$}<<ImageFileDirectory {}>>", " ", i, indent = indent);
    for ent in dir.entries.iter() {
      info!("{:indent$}- {:?}", " ", ent, indent = indent + 2);
    }
  }
}
