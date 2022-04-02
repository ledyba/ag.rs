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
  NoCompression, /* 1 */
  // (TIFF/EP p.30)
  // TIFF/EP readers are only required to support Baseline DCT JPEG method.
  BaselineJpeg, /* 7 */
  // https://www.awaresystems.be/imaging/tiff/tifftags/compression.html
  Undefined(u16)
}

#[derive(Clone, Debug)]
pub enum PhotometricInterpretation {
  Grayscale,
  RGB,
  YCbCr,
  ColorFilterArray,
  Undefined(u16),
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
pub enum PlanarConfiguration {
  Chunky,
  Planar,
  Unknown(u16)
}

#[derive(Clone, Debug)]
pub enum CFAPattern {
  R,
  G,
  B,
  Unknown(u8),
}

#[derive(Clone, Debug)]
pub enum YCbCrPositioning {
  CoSited,
  Undefined(u16)
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
  BitsPerSample(Vec<u16>),
  Compression(Compression),
  PhotometricInterpretation(PhotometricInterpretation),
  ImageDescription(String),
  Make(String),
  Model(String),
  StripOffsets(Vec<u32>),
  Orientation(Orientation),
  SamplesPerPixel(u16),
  RowsPerStrip(u32),
  StripByteCounts(Vec<u32>),
  XResolution(UnsignedRational),
  YResolution(UnsignedRational),
  PlanarConfiguration(PlanarConfiguration),
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
  YCbCrCoefficients {
    luma_red: UnsignedRational,
    luma_green: UnsignedRational,
    luma_blue: UnsignedRational,
  },
  YCbCrPositioning(YCbCrPositioning),
  CFARepeatPatternDim {
    rows: u16,
    cols: u16,
  },
  CFAPattern(Vec<CFAPattern>),
  DNGPrivateData(Vec<u8>),
  // Undefined
  Undefined(u16, DataType, u32, u32)
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
      if let &Entry::SubIFDs(ref vs) = ent {
        info!("{:indent$}- SubIFDs:", " ", indent = indent + 2);
        for (i, v) in vs.iter().enumerate() {
          self.inspect_dir(i as i32, v, indent + 4);
        }
      } else {
        info!("{:indent$}- {:?}", " ", ent, indent = indent + 2);
      }
    }
  }
}
