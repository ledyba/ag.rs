use log::{debug, info};
use super::*;

pub struct Parser {
  stream: Stream,
}

struct RawIFD {
  entries: Vec<RawEntry>
}

struct RawEntry {
  tag: u16,
  data_type: DataType,
  data_count: u32,
  data_or_offset: u32,
}

impl Parser {
  pub fn open(path: &str) -> anyhow::Result<Self> {
    let stream = Stream::open(path)?;
    Ok(Self{
      stream,
    })
  }

  pub fn parse(&mut self) -> anyhow::Result<Tiff> {
    let _ = self.stream.read_u16()?; // ignore header.
    let truth = self.stream.read_u16()?;
    if truth != 42 {
      return Err(anyhow::Error::msg("Not a TIFF file."));
    }
    let offset = self.stream.read_u32()?;
    self.stream.seek(offset as u64);
    let directories = self.parse_image_file_directories()?;
    Ok(Tiff{
      directories,
    })
  }

  fn parse_image_file_directories(&mut self) -> anyhow::Result<Vec<ImageFileDirectory>> {
    let mut ifd:Vec<ImageFileDirectory> = Vec::new();
    let mut pos = self.stream.position()?;
    while pos != 0 {
      self.stream.seek(pos)?;
      let mut entries = Vec::<Entry>::new();
      let num_entries = self.stream.read_u16()?;
      for i in 0..num_entries {
        entries.push(self.parse_entry()?);
      }
      ifd.push(ImageFileDirectory {
        entries
      });
      pos = self.stream.read_u32()? as u64;
    }
    Ok(ifd)
  }

  fn parse_entry(&mut self) -> anyhow::Result<Entry> {
    let tag = self.stream.read_u16()?;
    let data_type = DataType::from(self.stream.read_u16()?);
    let data_count = self.stream.read_u32()?;
    let data_or_offset = self.stream.read_u32()?;
    match tag {
      _ => {
        Ok(Entry::Unknown(tag, data_type, data_count, data_or_offset))
      }
    }
  }
}
