use std::io::{Read, Seek, SeekFrom};

pub struct Stream {
  file: std::fs::File,
}

#[derive(Copy, Clone)]
pub enum ByteOrder {
  LittleEndian,
  BigEndian,
}

impl Stream {
  pub fn open(path: &str) -> std::io::Result<Stream> {
    let file = std::fs::File::open(path)?;
    Ok(Self {
      file,
    })
  }
  pub fn read_byte(&mut self) -> std::io::Result<u8> {
    let mut buff = [0u8];
    self.file.read_exact(&mut buff)?;
    Ok(buff[0])
  }
  pub fn read_u16(&mut self, order: ByteOrder) -> std::io::Result<u16> {
    let mut buff = [0u8, 0u8];
    self.file.read_exact(&mut buff)?;
    match order {
      ByteOrder::LittleEndian => {
        Ok(buff[0] as u16 | ((buff[1] as u16) << 8))
      }
      ByteOrder::BigEndian => {
        Ok(buff[1] as u16 | ((buff[0] as u16) << 8))
      }
    }
  }
  pub fn read_u32(&mut self, order: ByteOrder) -> std::io::Result<u32> {
    let mut buff = [0u8, 0u8, 0u8, 0u8];
    self.file.read_exact(&mut buff)?;
    match order {
      ByteOrder::LittleEndian => {
        Ok(
          buff[0] as u32
            | ((buff[1] as u32) << 8)
            | ((buff[2] as u32) << 16)
            | ((buff[3] as u32) << 24)
        )
      }
      ByteOrder::BigEndian => {
        Ok(
          buff[3] as u32
            | ((buff[2] as u32) << 8)
            | ((buff[1] as u32) << 16)
            | ((buff[0] as u32) << 24)
        )
      }
    }
  }
  pub fn skip(&mut self, bytes: usize) -> std::io::Result<()> {
    self.file.seek(SeekFrom::Current(bytes as i64))?;
    Ok(())
  }
  pub fn seek(&mut self, offset: usize) -> std::io::Result<()> {
    self.file.seek(SeekFrom::Start(offset as u64))?;
    Ok(())
  }
}

#[cfg(test)]
mod test {
  use super::{Stream, ByteOrder};
  #[test]
  fn test_read() {
    let mut stream = Stream::open("sample/sample.arw").expect("Failed to open");
    let result = stream.read_byte().expect("Failed to read");
    assert_eq!(result, 0x49);
  }
  #[test]
  fn test_read_u16() {
    let mut stream = Stream::open("sample/sample.arw").expect("Failed to open");
    let result = stream.read_u16(ByteOrder::LittleEndian).expect("Failed to read");
    assert_eq!(result, 0x4949);
  }
  #[test]
  fn test_read_u32() {
    {
      let mut stream = Stream::open("sample/sample.arw").expect("Failed to open");
      stream.skip(4).expect("Failed to skip");
      let result = stream.read_u32(ByteOrder::LittleEndian).expect("Failed to read");
      assert_eq!(result, 0x08);
    }
    {
      let mut stream = Stream::open("sample/sample.arw").expect("Failed to open");
      stream.skip(4).expect("Failed to skip");
      let result = stream.read_u32(ByteOrder::BigEndian).expect("Failed to read");
      assert_eq!(result, 0x08000000);
    }
  }
}