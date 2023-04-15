/*
# Reference

libraw
- https://github.com/LibRaw/LibRaw/blob/2a9a4de21ea7f5d15314da8ee5f27feebf239655/src/decoders/decoders_dcraw.cpp#L1455

rawspeed
- https://github.com/darktable-org/rawspeed/blob/088ad10ca9c3064f0af892e87d50d5ca24481205/src/librawspeed/decompressors/SonyArw2Decompressor.h
- https://github.com/darktable-org/rawspeed/blob/088ad10ca9c3064f0af892e87d50d5ca24481205/src/librawspeed/decompressors/SonyArw2Decompressor.cpp

*/

use std::cmp::min;
use log::info;
use crate::stream::{BitStream, ByteStream};
use crate::raw::RawImage;
use crate::tiff::{CFAPatternDim, CFAPattern, Tiff};

pub struct Arw2Decompressor<'a> {
  stream: &'a mut ByteStream,
  tiff: &'a Tiff,
  width: usize,
  height: usize,
  data_offset: usize,
  data_size: usize,
  cfa_pattern: &'a Vec<CFAPattern>,
  cfa_dim: CFAPatternDim,
}

impl <'a> Arw2Decompressor<'a> {
  pub fn new(
    stream: &'a mut ByteStream,
    tiff: &'a Tiff,
    width: usize,
    height: usize,
    data_offset: usize,
    data_size: usize,
    cfa_pattern: &'a Vec<CFAPattern>,
    cfa_dim: CFAPatternDim,
  ) -> Self {
    Self {
      stream,
      tiff,
      width,
      height,
      data_offset,
      data_size,
      cfa_pattern,
      cfa_dim,
    }
  }

  pub fn decode(&mut self) -> Result<RawImage, anyhow::Error> {
    let mut img = RawImage::new(
      self.width,
      self.height,
      self.cfa_pattern.clone(),
      self.cfa_dim.clone(),
    );
    for y in 0..self.height {
      let offset = self.data_offset + self.width * y;
      self.stream.seek(offset as u64)?;
      let mut bits = BitStream::new(&mut self.stream);
      let mut x = 0;
      while x < self.width {
        let max = bits.read_bits(11)?;
        let min = bits.read_bits(11)?;
        let i_max = bits.read_bits(4)?;
        let i_min = bits.read_bits(4)?;
        if i_max == i_min {
          return Err(anyhow::Error::msg("ARW2 invariant failed, same pixel is both min and max"))
        }
        let mut sh = 0;
        while (sh < 4) && ((0x80 << sh) <= (max - min)) {
          sh += 1;
        }
        for i in 0..16 {
          let p =
          if i == i_max {
            max
          } else {
            if i == i_min {
              min
            } else {
              let p = (bits.read_bits(7)? << sh) + min;
              std::cmp::min(0x7ff, p)
            }
          };
          img.set(x + (i*2) as usize, y,(p << 1) as u16);
        }
        if (x & 1) != 0 {
          x += 31;
        } else {
          x += 1;
        }
      }
    }
    Ok(img)
  }
}
