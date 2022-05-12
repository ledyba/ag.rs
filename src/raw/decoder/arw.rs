use std::fmt::format;
use log::info;
use crate::raw::Arw2Decompressor;
use crate::raw::decoder::Image;
use crate::tiff::{Compression, Entry, Tiff};
use crate::stream::ByteStream;
use super::RawDecoder;

/*
References:

rawspeed:
- https://github.com/darktable-org/rawspeed/blob/1a1b723420bd3c923b0ed242287e6c615cd87af4/src/librawspeed/decoders/ArwDecoder.cpp

libraw:
- https://github.com/LibRaw/LibRaw/blob/adcb898a00746c8aa886eb06cc9f5a1cb1834fca/src/metadata/tiff.cpp#L1815-L1838
*/

pub struct ArwDecoder<'a> {
  stream: &'a mut ByteStream,
  tiff: &'a Tiff,
}

impl <'a> ArwDecoder<'a>  {
  pub fn new(stream: &'a mut ByteStream, tiff: &'a Tiff) -> Self {
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

  fn decode(&mut self) -> Result<Image, anyhow::Error> {
    let ifds = self.tiff.filter_ifd_recursive(|it|
      it.find(|e|
        if let Entry::StripOffsets(_) = e {
          Some(())
        } else {
          None
        }
      ).is_some()
    );
    if ifds.is_empty() {
      return Err(anyhow::Error::msg("No IFDs"));
    }
    let ifd = ifds[0];
    let mut compression = ifd.compression();
    let mut width = ifd.image_width();
    let mut height = ifd.image_height();
    let mut offsets = ifd.strip_byte_offsets();
    let mut counts = ifd.strip_byte_counts();
    let mut bpp = ifd.bits_per_sample();
    for ent in ifd.entries() {
      match ent {
        _ => {}
      }
    }

    if compression != Some(Compression::SonyARW) {
      return Err(anyhow::Error::msg(format!("Unsupported compression type: {:?}", compression)));
    }
    if width.is_none() {
      return Err(anyhow::Error::msg("No width"));
    }
    let width = width.unwrap();
    if height.is_none() {
      return Err(anyhow::Error::msg("No height"));
    }
    let mut height = height.unwrap();
    if offsets.is_none() {
      return Err(anyhow::Error::msg("No strip byte offsets"));
    }
    let offsets = offsets.unwrap();
    if counts.is_none() {
      return Err(anyhow::Error::msg("No strip byte counts"));
    }
    let counts = counts.unwrap();
    if bpp.is_none() {
      return Err(anyhow::Error::msg("No bits per sample"));
    }
    let bpp = bpp.unwrap();
    if offsets.len() != 1 {
      return Err(anyhow::Error::msg("Multiple strips found"));
    }
    if offsets.len() != counts.len() {
      return Err(
        anyhow::Error::msg(
          format!(
            "Byte count number does not match strip size: count:{}, strips:{}",
            offsets.len(),
            counts.len())));
    }
    let offset = offsets[0];
    let count = counts[0];
    let mut bpp = bpp[0];
    for ifd in self.tiff.filter_ifd_recursive(|it| it.make().is_some()) {
      if ifd.make().unwrap() == "SONY" {
        bpp = 8;
      }
    }
    let is_v1 = (count as usize) * 8 != (width as usize) * (height as usize) * (bpp as usize);
    if is_v1 {
      height += 8;
      return Err(anyhow::Error::msg("ARW v1 is not supported"));
    }
    let mut decoder = Arw2Decompressor::new(
      self.stream,
      self.tiff,
      width as usize,
      height as usize,
      offset as usize,
      count as usize,
    );
    decoder.decode()
  }
}
