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
    let data_or_offset = self.stream.read_u32()?;
    let check = |types: &[DataType]| -> anyhow::Result<()> {
      for ty in types {
        if *ty == data_type {
          return Ok(());
        }
      }
      let msg = format!("Type Mismatch: {:?} not in {:?}", data_type, types);
      Err(anyhow::Error::msg(msg))
    };
    // See p.117
    // https://www.adobe.io/content/dam/udp/en/open/standards/tiff/TIFF6.pdf
    let entry = match tag {
      254 => {
        // p.36
        check(&[DataType::U32])?;
        Entry::NewSubFileType {
          is_reduced: (data_or_offset & 1) == 1,
          is_single_page_of_multi_page: (data_or_offset & 2) == 2,
          is_transparency_mask_for_another: (data_or_offset & 4) == 4,
        }
      },
      255 => {
        // p.40
        check(&[DataType::U16])?;
        match data_or_offset {
          1 => Entry::SubFileType(SubFileType::FullResolution),
          2 => Entry::SubFileType(SubFileType::ReducedResolution),
          3 => Entry::SubFileType(SubFileType::SinglePageOfMultiPage),
          _ => {
            return Err(anyhow::Error::msg("Invalid SubFileType"))
          }
        }
      },
      256 => {
        // p.18
        check(&[DataType::U16, DataType::U32])?;
        Entry::ImageWidth(data_or_offset)
      },
      257 => {
        // p.18
        check(&[DataType::U16, DataType::U32])?;
        Entry::ImageLength(data_or_offset)
      },
      258 => {
        Entry::BitsPerSample
      },
      259 => {
        Entry::Compression
      },
      262 => {
        Entry::PhotometricInterpretation
      },
      263 => {
        Entry::Thresholding
      },
      _ => {
        Entry::Unknown(tag, data_type, data_count, data_or_offset)
      }
    };
    Ok(entry)
  }
}
