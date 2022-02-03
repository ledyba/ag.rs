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
    let data_type = DataType::from(self.stream.read_u16()?);
    let data_count = self.stream.read_u32()?;
    // data or offset
    let data_offset = self.stream.position()?;
    let data = self.stream.read_u32()?;
    let offset = data as u64;
    /* ***********************************************************************
     * util functions
     * ***********************************************************************/
    let check_type = |types: &[DataType]| -> anyhow::Result<()> {
      for ty in types {
        if *ty == data_type {
          return Ok(());
        }
      }
      let msg = format!("Type Mismatch: {:?} not in {:?}", data_type, types);
      Err(anyhow::Error::msg(msg))
    };
    let mut read_ascii = || {
      if data_count > 4 {
        self.stream.fetch_ascii(offset, data_count as usize)
      } else {
        self.stream.fetch_ascii(data_offset, data_count as usize)
      }
    };
    let mut read_binary = || {
      if data_count > 4 {
        self.stream.fetch_vec_u8(offset, data_count as usize)
      } else {
        self.stream.fetch_vec_u8(data_offset, data_count as usize)
      }
    };
    /* ************************************************************************
     * Analyze via tag
     * See p.17 for correspondence between tag name and value.
     *************************************************************************/
    let entry = match tag {
      254 => {
        // p.20
        check_type(&[DataType::U32])?;
        Entry::NewSubFileType {
          is_thumbnail: (data & 1) == 1,
        }
      }
      256 => {
        // p.20
        check_type(&[DataType::U16, DataType::U32])?;
        Entry::ImageWidth(data)
      }
      257 => {
        // p.20
        check_type(&[DataType::U16, DataType::U32])?;
        Entry::ImageLength(data)
      }
      258 => {
        // TODO
        Entry::BitsPerSample
      }
      259 => {
        // p.30
        check_type(&[DataType::U16])?;
        match data {
          1 => Entry::Compression(Compression::NoCompression),
          7 => Entry::Compression(Compression::Jpeg),
          _ => Entry::Compression(Compression::Unknown(data as u16)),
        }
      }
      262 => {
        // TODO
        Entry::PhotometricInterpretation
      }
      270 => {
        check_type(&[DataType::Ascii])?;
        let description = read_ascii()?;
        Entry::ImageDescription(description)
      }
      271 => {
        check_type(&[DataType::Ascii])?;
        Entry::Make(read_ascii()?)
      }
      272 => {
        check_type(&[DataType::Ascii])?;
        Entry::Model(read_ascii()?)
      }
      274 => {
        check_type(&[DataType::U16])?;
        match data {
          1 => Entry::Orientation(Orientation::Rotate0),
          3 => Entry::Orientation(Orientation::Rotate180),
          6 => Entry::Orientation(Orientation::Rotate270),
          8 => Entry::Orientation(Orientation::Rotate90),
          _ => Entry::Orientation(Orientation::Undefined(data as u16)),
        }
      }
      282 => {
        check_type(&[DataType::Rational])?;
        Entry::XResolution(self.stream.fetch_unsigned_rational(offset)?)
      }
      283 => {
        check_type(&[DataType::Rational])?;
        Entry::YResolution(self.stream.fetch_unsigned_rational(offset)?)
      }
      296 => { // p.22
        check_type(&[DataType::U16])?;
        match data {
          1 => Entry::ResolutionUnit(ResolutionUnit::Unknown),
          2 => Entry::ResolutionUnit(ResolutionUnit::Inch),
          3 => Entry::ResolutionUnit(ResolutionUnit::Centimeter),
          _ => Entry::ResolutionUnit(ResolutionUnit::Undefined(data as u16)),
        }
      }
      305 => {
        check_type(&[DataType::Ascii])?;
        Entry::Software(read_ascii()?)
      }
      306 => {
        check_type(&[DataType::Ascii])?;
        Entry::DateTime(read_ascii()?)
      }
      _ => {
        warn!("Unknown Tag: {}", tag);
        Entry::Unknown(tag, data_type, data_count, data)
      }
    };
    Ok(entry)
  }
}
