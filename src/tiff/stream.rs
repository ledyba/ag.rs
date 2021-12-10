use std::io::Read;

pub struct Stream {
  file: std::fs::File,
}

impl Stream {
  fn open(path: &str) -> std::io::Result<Stream> {
    let file = std::fs::File::open(path)?;
    Ok(Self {
      file,
    })
  }
  fn read_byte(&mut self) -> std::io::Result<u8> {
    let mut buff = [0u8];
    self.file.read_exact(&mut buff)?;
    Ok(buff[0])
  }
}

#[cfg(test)]
mod test {
  use super::Stream;
  #[test]
  fn test_read() {
    let mut stream = Stream::open("sample/sample.arw").expect("Failed to open");
    let result = stream.read_byte().expect("Failed to read");
    assert_eq!(result, 0x49);
  }
}