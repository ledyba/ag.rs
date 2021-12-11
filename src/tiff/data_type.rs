#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum DataType {
  U8,
  Ascii,
  U16,
  U32,
  Rational,
  S8,
  Blob,
  S16,
  S32,
  SRational,
  F32,
  F64,
  Unknown(u16),
}

impl From<u16> for DataType {
  fn from(code: u16) -> Self {
    match code {
      1 => DataType::U8,
      2 => DataType::Ascii,
      3 => DataType::U16,
      4 => DataType::U32,
      5 => DataType::Rational,
      6 => DataType::S8,
      7 => DataType::Blob,
      8 => DataType::S16,
      9 => DataType::S32,
      10 => DataType::SRational,
      11 => DataType::F32,
      12 => DataType::F64,
      _ => DataType::Unknown(code),
    }
  }
}
