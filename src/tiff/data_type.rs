#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum TypeTag {
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

impl TypeTag {
  fn size(&self) -> usize {
    match *self {
      TypeTag::U8 => 1,
      TypeTag::Ascii => 1,
      TypeTag::U16 => 2,
      TypeTag::U32 => 4,
      TypeTag::Rational => 8,
      TypeTag::S8 => 1,
      TypeTag::Blob => 1,
      TypeTag::S16 => 2,
      TypeTag::S32 => 4,
      TypeTag::SRational => 8,
      TypeTag::F32 => 4,
      TypeTag::F64 => 8,
      TypeTag::Unknown(_) => 1,
    }
  }
}

impl From<u16> for TypeTag {
  fn from(code: u16) -> Self {
    match code {
      1 => TypeTag::U8,
      2 => TypeTag::Ascii,
      3 => TypeTag::U16,
      4 => TypeTag::U32,
      5 => TypeTag::Rational,
      6 => TypeTag::S8,
      7 => TypeTag::Blob,
      8 => TypeTag::S16,
      9 => TypeTag::S32,
      10 => TypeTag::SRational,
      11 => TypeTag::F32,
      12 => TypeTag::F64,
      _ => TypeTag::Unknown(code),
    }
  }
}

pub struct UnsignedRational {
  pub numerator: u32,
  pub denominator: u32,
}

pub struct SignedRational {
  pub numerator: i32,
  pub denominator: i32,
}
