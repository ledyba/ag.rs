use crate::raw::decoder::Image;
use crate::tiff::Tiff;
use crate::tiff::Stream;
use super::RawDecoder;

/*
References:
https://github.com/darktable-org/rawspeed/blob/1a1b723420bd3c923b0ed242287e6c615cd87af4/src/librawspeed/decoders/ArwDecoder.cpp
*/

pub struct ArwDecoder<'a> {
  stream: &'a Stream,
  tiff: &'a Tiff,
}

impl <'a> ArwDecoder<'a>  {
  pub fn new(stream: &'a Stream, tiff: &'a Tiff) -> Self {
    Self {
      stream,
      tiff,
    }
  }
}

impl <'a> RawDecoder for ArwDecoder<'a> {
  fn is_acceptable(&self) -> bool {
    if let Some(ifd) = self.tiff.root_ifd() {
      return ifd.make() == Some("SONY");
    }
    false
  }

  fn decode(&self) -> Image {
    todo!()
  }
}