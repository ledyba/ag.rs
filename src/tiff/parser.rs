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
  data_type: TypeTag,
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
    let mut raw:Vec<RawIFD> = Vec::new();
    self.parse_ifd_raw(&mut raw);
    unimplemented!();
  }

  fn parse_ifd_raw(&mut self, directories: &mut Vec<RawIFD>) -> anyhow::Result<()> {
    let num_entries = self.stream.read_u16()?;
    let mut entries = Vec::<RawEntry>::new();
    for i in 0..num_entries {
      let tag = self.stream.read_u16()?;
      let data_type = TypeTag::from(self.stream.read_u16()?);
      let data_count = self.stream.read_u32()?;
      let data_or_offset = self.stream.read_u32()?;
      debug!("tag: {:?} type: {:?} count: {} val: {}", tag, data_type, data_count, data_or_offset);
      entries.push(RawEntry{
        tag,
        data_type,
        data_count,
        data_or_offset,
      });
    }
    directories.push(RawIFD {
      entries,
    });
    let offset = self.stream.read_u32()?;
    if offset != 0 {
      debug!("Additional IFD at {}", offset);
      self.stream.seek(offset as u64);
      return self.parse_ifd_raw(directories);
    }
    debug!("All IFDs are parsed.");
    Ok(())
  }
}
