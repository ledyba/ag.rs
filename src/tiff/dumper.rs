use std::fmt::format;
use std::fs::File;
use std::io::Write;
use crate::tiff::{Entry, ImageFileDirectory, Stream, Tiff};

pub struct Dumper <'a> {
  stream: &'a mut Stream,
  image: &'a Tiff,
}

impl <'a> Dumper <'a> {
  pub fn new(stream: &'a mut Stream,image: &'a Tiff) -> Self {
    Self {
      stream,
      image,
    }
  }
  pub fn dump(&mut self) -> anyhow::Result<()> {
    self.dump_directories("", &self.image.directories)
  }
  fn dump_directories(&mut self, levels: &str, dirs: &Vec<ImageFileDirectory>)
    -> anyhow::Result<()>
  {
    for (idx, dir) in dirs.iter().enumerate() {
      let mut offsets: Option<Vec<u32>> = None;
      let mut bytes: Option<Vec<u32>> = None;
      for e in dir.entries.iter() {
        match e {
          Entry::StripOffsets(vs) => {
            offsets = Some(vs.clone());
          }
          Entry::StripByteCounts(vs) => {
            bytes = Some(vs.clone());
          }
          Entry::SubIFDs(dirs) => {
            let next_level = if levels == "" {
              format!("{}", idx)
            } else {
              format!("{}-{}", levels, idx)
            };
            self.dump_directories(&next_level, dirs);
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
        for (idx, (offset, bytes)) in offsets.iter().zip(bytes.iter()).enumerate() {
          self.stream.seek(*offset as u64)?;
          let data = self.stream.read_vec_u8(*bytes as usize)?;
          let mut f = File::create(format!("{}_{}.dump", levels, idx))?;
          f.write_all(&data)?;
        }
      } else {
        return Err(anyhow::Error::msg("Both StripOffsets and StripByteCounts must be set."));
      }
    }
    Ok(())
  }
}