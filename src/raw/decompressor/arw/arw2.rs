/*
# Reference

libraw
- https://github.com/LibRaw/LibRaw/blob/2a9a4de21ea7f5d15314da8ee5f27feebf239655/src/decoders/decoders_dcraw.cpp#L1455

rawspeed
- https://github.com/darktable-org/rawspeed/blob/088ad10ca9c3064f0af892e87d50d5ca24481205/src/librawspeed/decompressors/SonyArw2Decompressor.h
- https://github.com/darktable-org/rawspeed/blob/088ad10ca9c3064f0af892e87d50d5ca24481205/src/librawspeed/decompressors/SonyArw2Decompressor.cpp

*/

use crate::stream::ByteStream;
use crate::raw::Image;
use crate::tiff::Tiff;

pub struct Arw2Decompressor<'a> {
  stream: &'a mut ByteStream,
  tiff: &'a Tiff,
  width: usize,
  height: usize,
  data_offset: usize,
  data_size: usize,
}

impl <'a> Arw2Decompressor<'a> {
  pub fn new(
    stream: &'a mut ByteStream,
    tiff: &'a Tiff,
    width: usize,
    height: usize,
    data_offset: usize,
    data_size: usize,
  ) -> Self {
    Self {
      stream,
      tiff,
      width,
      height,
      data_offset,
      data_size,
    }
  }

  pub fn decode(&self) -> Result<Image, anyhow::Error> {
    todo!()
  }
}
