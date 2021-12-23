#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum Tag {
  // See p.117
  // https://www.adobe.io/content/dam/udp/en/open/standards/tiff/TIFF6.pdf
  NewSubFileType,
  SubFileType,
  ImageWidth,
  ImageHeight,
  BitsPerSample,
  Compression,
  PhotometricInterpretation,
  Thresholding,
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
      263 => Thresholding,
      _ => Unknown(code),
    }
  }
}
