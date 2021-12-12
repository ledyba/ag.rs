#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum Tag {
  NewSubFileType,
  SubFileType,
  ImageWidth,
  ImageHeight,
  BitsPerSample,
  Compression,
  PhotometricInterpretation,
  Unknown(u16),
}

impl From<u16> for Tag {
  fn from(code: u16) -> Self {
    use Tag::*;
    match code {
      254 => NewSubFileType,
      255 => SubFileType,
      256 => ImageWidth,
      257 => ImageWidth,
      258 => BitsPerSample,
      259 => Compression,
      262 => PhotometricInterpretation,
      _ => Unknown(code),
    }
  }
}
