use log::warn;
use super::*;

pub struct Parser {
  stream: Stream,
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
    self.stream.seek(offset as u64)?;
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
      for _ in 0..num_entries {
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
    let mut ctx = {
      let ty = DataType::from(self.stream.read_u16()?);
      let count = self.stream.read_u32()?;
      // data or offset
      let data_offset = self.stream.position()?;
      let data = self.stream.read_u32()?;
      EntryContext {
        stream: &mut self.stream,
        ty,
        count,
        data_offset,
        data,
      }
    };
    /* ************************************************************************
     * Analyze via tag
     * See p.17 for correspondence between tag name and value.
     *************************************************************************/
    let entry = match tag {
      254 => {
        // p.20
        ctx.check_type(&[DataType::U32])?;
        Entry::NewSubFileType {
          is_thumbnail: (ctx.data & 1) == 1,
        }
      }
      256 => {
        // p.20
        ctx.check_type(&[DataType::U16, DataType::U32])?;
        Entry::ImageWidth(ctx.data)
      }
      257 => {
        // p.20
        ctx.check_type(&[DataType::U16, DataType::U32])?;
        Entry::ImageLength(ctx.data)
      }
      258 => {
        // TODO
        Entry::BitsPerSample
      }
      259 => {
        // p.30
        ctx.check_type(&[DataType::U16])?;
        match ctx.data {
          1 => Entry::Compression(Compression::NoCompression),
          7 => Entry::Compression(Compression::Jpeg),
          _ => Entry::Compression(Compression::Unknown(ctx.data as u16)),
        }
      }
      262 => {
        // TODO
        Entry::PhotometricInterpretation
      }
      270 => {
        ctx.check_type(&[DataType::Ascii])?;
        let description = ctx.read_ascii()?;
        Entry::ImageDescription(description)
      }
      271 => {
        ctx.check_type(&[DataType::Ascii])?;
        Entry::Make(ctx.read_ascii()?)
      }
      272 => {
        ctx.check_type(&[DataType::Ascii])?;
        Entry::Model(ctx.read_ascii()?)
      }
      274 => {
        ctx.check_type(&[DataType::U16])?;
        match ctx.data {
          1 => Entry::Orientation(Orientation::Rotate0),
          3 => Entry::Orientation(Orientation::Rotate180),
          6 => Entry::Orientation(Orientation::Rotate270),
          8 => Entry::Orientation(Orientation::Rotate90),
          _ => Entry::Orientation(Orientation::Undefined(ctx.data as u16)),
        }
      }
      282 => {
        ctx.check_type(&[DataType::Rational])?;
        Entry::XResolution(ctx.stream.fetch_unsigned_rational(ctx.data as u64)?)
      }
      283 => {
        ctx.check_type(&[DataType::Rational])?;
        Entry::YResolution(ctx.stream.fetch_unsigned_rational(ctx.data as u64)?)
      }
      296 => { // p.22
        ctx.check_type(&[DataType::U16])?;
        match ctx.data {
          1 => Entry::ResolutionUnit(ResolutionUnit::Unknown),
          2 => Entry::ResolutionUnit(ResolutionUnit::Inch),
          3 => Entry::ResolutionUnit(ResolutionUnit::Centimeter),
          _ => Entry::ResolutionUnit(ResolutionUnit::Undefined(ctx.data as u16)),
        }
      }
      305 => {
        ctx.check_type(&[DataType::Ascii])?;
        Entry::Software(ctx.read_ascii()?)
      }
      306 => {
        ctx.check_type(&[DataType::Ascii])?;
        Entry::DateTime(ctx.read_ascii()?)
      }
      _ => {
        warn!("Unknown Tag: {}", tag);
        Entry::Unknown(tag, ctx.ty, ctx.count, ctx.data)
      }
    };
    Ok(entry)
  }
}

struct EntryContext<'s> {
  stream: &'s mut Stream,
  ty: DataType,
  count: u32,
  data_offset: u64,
  data: u32,
}

impl <'s> EntryContext<'s> {
  fn check_type(&self, types: &[DataType]) -> anyhow::Result<()> {
    for ty in types {
      if *ty == self.ty {
      return Ok(());
      }
    }
    let msg = format!("Type Mismatch: {:?} not in {:?}", self.ty, types);
    Err(anyhow::Error::msg(msg))
  }
  fn read_ascii(&mut self) -> std::io::Result<String> {
    if self.count > 4 {
      self.stream.fetch_ascii(self.data as u64, self.count as usize)
    } else {
      self.stream.fetch_ascii(self.data_offset, self.count as usize)
    }
  }
  fn read_binary(&mut self) -> std::io::Result<Vec<u8>> {
    if self.count > 4 {
      self.stream.fetch_vec_u8(self.data as u64, self.count as usize)
    } else {
      self.stream.fetch_vec_u8(self.data_offset, self.count as usize)
    }
  }

}