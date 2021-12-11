use log::info;
use super::{Stream, ByteOrder, Tiff, Entry};
pub struct Parser {
  stream: Stream,
  byte_order: ByteOrder,
}

impl Parser {
  pub fn open(path: &str) -> std::io::Result<Self> {
    let stream = Stream::open(path)?;
    Ok(Self{
      stream,
      byte_order: ByteOrder::LittleEndian,
    })
  }
  pub fn parse(&mut self) -> anyhow::Result<Tiff> {
    self.byte_order = match self.stream.read_u16(ByteOrder::LittleEndian)? {
      0x4D4D => ByteOrder::BigEndian,
      0x4949 => ByteOrder::LittleEndian,
      _ => return Err(anyhow::Error::msg("Not a TIFF file.")),
    };
    let truth = self.read_u16()?;
    if truth != 42 {
      return Err(anyhow::Error::msg("Not a TIFF file."));
    }
    let offset = self.read_u32()?;
    self.stream.seek(offset as usize);
    let mut entries = Vec::<Entry>::new();
    self.parse_image_file_directories(&mut entries)?;
    Ok(Tiff{
      entries,
    })
  }
  fn read_u16(&mut self) -> std::io::Result<u16> {
    self.stream.read_u16(self.byte_order)
  }
  fn read_u32(&mut self) -> std::io::Result<u32> {
    self.stream.read_u32(self.byte_order)
  }
  fn parse_image_file_directories(&mut self, entries: &mut Vec<Entry>) -> anyhow::Result<()> {
    let num_entries = self.read_u16()?;
    for i in 0..num_entries {
      let tag = self.read_u16()?;
      let data_type = self.read_u16()?;
      let data_count = self.read_u32()?;
      let data_or_offset = self.read_u32()?;
      info!("tag: {} type: {} count: {} val: {}", tag, data_type, data_count, data_or_offset);
    }
    let offset = self.read_u32()?;
    if offset != 0 {
      self.stream.seek(offset as usize);
      return self.parse_image_file_directories(entries);
    }
    Ok(())
  }
}
