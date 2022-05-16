use std::fs::File;
use std::io::Write;

use crate::stream::ByteStream;
use crate::tiff::{Entry, ImageFileDirectory, Tiff};

pub struct Dumper <'a> {
  stream: &'a mut ByteStream,
  image: &'a Tiff,
}

impl <'a> Dumper <'a> {
  pub fn new(stream: &'a mut ByteStream, image: &'a Tiff) -> Self {
    Self {
      stream,
      image,
    }
  }
  pub fn dump(&mut self) -> anyhow::Result<()> {
    self.dump_directories("", &self.image.directories)
  }
  fn dump_directories(&mut self, depth: &str, dirs: &Vec<ImageFileDirectory>)
    -> anyhow::Result<()>
  {
    for (idx, dir) in dirs.iter().enumerate() {
      let next_depth = if depth == "" {
        format!("{}", idx)
      } else {
        format!("{}-{}", depth, idx)
      };
      let mut offsets: Option<Vec<u32>> = None;
      let mut bytes: Option<Vec<u32>> = None;
      let mut strip_idx = 0 as usize;
      for e in dir.entries.iter() {
        match e {
          Entry::StripOffsets(vs) => {
            offsets = Some(vs.clone());
          }
          Entry::StripByteCounts(vs) => {
            bytes = Some(vs.clone());
          }
          Entry::SubIFDs(dirs) => {
            self.dump_directories(&next_depth, dirs)?;
          }
          _ => {}
        }
      }
      if offsets.is_none() && bytes.is_none() {
        continue;
      }
      if let (Some(offsets), Some(bytes)) = (offsets, bytes) {
        if offsets.len() != bytes.len() {
          return Err(anyhow::Error::msg("StripOffsets.len != StripByteCounts.len"));
        }
        for (_idx, (offset, bytes)) in offsets.iter().zip(bytes.iter()).enumerate() {
          self.stream.seek(*offset as u64)?;
          let data = self.stream.read_vec_u8(*bytes as usize)?;
          let mut f = File::create(format!("{}_{}.dump", next_depth, strip_idx))?;
          f.write_all(&data)?;
        }
      } else {
        return Err(anyhow::Error::msg("Both StripOffsets and StripByteCounts must be set."));
      }
    }
    Ok(())
  }
}
