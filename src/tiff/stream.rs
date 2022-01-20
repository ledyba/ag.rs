use std::io::{Read, Seek, SeekFrom};
use byteordered::{Endian, Endianness};
use byteordered::byteorder::ReadBytesExt;
use super::data_type::{UnsignedRational, SignedRational};

pub struct Stream {
  endian: Endianness,
  file: std::fs::File,
}

impl Stream {
  pub fn open(path: &str) -> anyhow::Result<Stream> {
    let mut file = std::fs::File::open(path)?;
    let endian = {
      let mut header: [u8; 2] = [0, 0];
      file.read_exact(&mut header)?;
      if header == [0x4D, 0x4D] {
        Endianness::Big
      } else if header == [0x49, 0x49] {
        Endianness::Little
      } else {
        return Err(anyhow::Error::msg("Not a TIFF file."));
      }
    };
    file.seek(SeekFrom::Start(0))?;
    Ok(Self {
      endian,
      file,
    })
  }

  /* u8 */
  pub fn read_u8(&mut self) -> std::io::Result<u8> {
    self.file.read_u8()
  }
  pub fn read_u8s(&mut self, n: usize) -> std::io::Result<Vec<u8>> {
    let mut buff = Vec::<u8>::new();
    buff.resize(n, 0);
    self.file.read_exact(&mut buff)?;
    Ok(buff)
  }

  /* u8 */
  pub fn read_i8(&mut self) -> std::io::Result<i8> {
    self.file.read_i8()
  }
  pub fn read_i8s(&mut self, n: usize) -> std::io::Result<Vec<i8>> {
    let mut buff = Vec::<i8>::new();
    buff.resize(n, 0);
    self.file.read_i8_into(&mut buff)?;
    Ok(buff)
  }

  /* u16 */
  pub fn read_u16(&mut self) -> std::io::Result<u16> {
    self.endian.read_u16(&mut self.file)
  }
  pub fn read_u16s(&mut self, n: usize) -> std::io::Result<Vec<u16>> {
    let mut buff: Vec<u16> = Vec::new();
    buff.resize(n, 0);
    self.endian.read_u16_into(&mut self.file, &mut buff)?;
    Ok(buff)
  }

  /* s16 */
  pub fn read_i16(&mut self) -> std::io::Result<i16> {
    self.endian.read_i16(&mut self.file)
  }
  pub fn read_i16s(&mut self, n: usize) -> std::io::Result<Vec<i16>> {
    let mut buff: Vec<i16> = Vec::new();
    buff.resize(n, 0);
    self.endian.read_i16_into(&mut self.file, &mut buff)?;
    Ok(buff)
  }

  /* u32 */
  pub fn read_u32(&mut self) -> std::io::Result<u32> {
    self.endian.read_u32(&mut self.file)
  }
  pub fn read_u32s(&mut self, n: usize) -> std::io::Result<Vec<u32>> {
    let mut buff: Vec<u32> = Vec::new();
    buff.resize(n, 0);
    self.endian.read_u32_into(&mut self.file, &mut buff)?;
    Ok(buff)
  }

  /* s32 */
  pub fn read_i32(&mut self) -> std::io::Result<i32> {
    self.endian.read_i32(&mut self.file)
  }
  pub fn read_i32s(&mut self, n: usize) -> std::io::Result<Vec<i32>> {
    let mut buff: Vec<i32> = Vec::new();
    buff.resize(n, 0);
    self.endian.read_i32_into(&mut self.file, &mut buff)?;
    Ok(buff)
  }

  /* f32 */
  pub fn read_f32(&mut self) -> std::io::Result<f32> {
    self.endian.read_f32(&mut self.file)
  }
  pub fn read_f32s(&mut self, n: usize) -> std::io::Result<Vec<f32>> {
    let mut buff: Vec<f32> = Vec::new();
    buff.resize(n, 0.0);
    self.endian.read_f32_into(&mut self.file, &mut buff)?;
    Ok(buff)
  }

  /* f64 */
  pub fn read_f64(&mut self) -> std::io::Result<f64> {
    self.endian.read_f64(&mut self.file)
  }
  pub fn read_f64s(&mut self, n: usize) -> std::io::Result<Vec<f64>> {
    let mut buff: Vec<f64> = Vec::new();
    buff.resize(n, 0.0);
    self.endian.read_f64_into(&mut self.file, &mut buff)?;
    Ok(buff)
  }

  /* rational */
  pub fn read_unsigned_rational(&mut self) -> std::io::Result<UnsignedRational> {
    let mut buff: [u32; 2] = [0, 0];
    self.endian.read_u32_into(&mut self.file, &mut buff)?;
    Ok(UnsignedRational {
      numerator: buff[0],
      denominator: buff[1],
    })
  }
  pub fn read_unsigned_rationals(&mut self, n: usize) -> std::io::Result<Vec<UnsignedRational>> {
    let buff = self.read_u32s(n * 2)?;
    let values: Vec<UnsignedRational> = buff.chunks(2).map(|v| UnsignedRational {
      numerator: v[0],
      denominator: v[1],
    }).collect();
    Ok(values)
  }

  /* SRational */
  pub fn read_signed_rational(&mut self) -> anyhow::Result<SignedRational> {
    let mut buff: [i32; 2] = [0, 0];
    s.endian.read_i32_into(&mut s.file, &mut buff)?;
    Ok(SignedRational {
      numerator: buff[0],
      denominator: buff[1],
    })
  }
  pub fn fetch_signed_rational(&mut self, offset: u64) -> anyhow::Result<SignedRational> {
    self.warp(offset, |s| {
      s.read_signed_rational()
    })
  }
  pub fn fetch_signed_rationals(&mut self, offset: u64, n: usize) -> anyhow::Result<Vec<SignedRational>> {
    self.warp(offset, |s| {
      let buff = s.read_i32s(n * 2)?;
      let values: Vec<SignedRational> = buff.chunks_exact(2).map(|v| SignedRational {
        numerator: v[0],
        denominator: v[1],
      }).collect();
      Ok(values)
    })
  }

  /* Skip */
  pub fn skip(&mut self, bytes: i64) -> std::io::Result<()> {
    self.file.seek(SeekFrom::Current(bytes))?;
    Ok(())
  }

  pub fn seek(&mut self, offset: u64) -> std::io::Result<()> {
    self.file.seek(SeekFrom::Start(offset))?;
    Ok(())
  }

  pub fn position(&mut self) -> std::io::Result<u64> {
    self.file.stream_position()
  }

  pub fn warp<'s, Fn, T>(&'s mut self, offset: u64, f: Fn) -> anyhow::Result<T>
    where
    for<'f> Fn: FnOnce(&'f mut Self) -> anyhow::Result<T>,
  {
    self.fork(|stream| {
      stream.seek(offset)?;
      f(stream)
    })
  }

  pub fn fork<'s, Fn, T>(&'s mut self, f: Fn) -> anyhow::Result<T>
    where
    // https://doc.rust-lang.org/reference/trait-bounds.html#higher-ranked-trait-bounds
    for <'f> Fn: FnOnce(&'f mut Self) -> anyhow::Result<T>,
  {
    let original_offset = self.position()?;
    let r = f(self);
    self.seek(original_offset)?;
    r
  }
}

#[cfg(test)]
mod test {
  use super::Stream;

  #[test]
  fn test_read() {
    let mut stream = Stream::open("sample/sample.arw").expect("Failed to open");
    let result = stream.read_u8().expect("Failed to read");
    assert_eq!(result, 0x49);
  }

  #[test]
  fn test_read_u16() {
    let mut stream = Stream::open("sample/sample.arw").expect("Failed to open");
    let result = stream.read_u16().expect("Failed to read");
    assert_eq!(result, 0x4949);
  }

  #[test]
  fn test_read_u32() {
    let mut stream = Stream::open("sample/sample.arw").expect("Failed to open");
    stream.skip(4).expect("Failed to skip");
    let result = stream.read_u32().expect("Failed to read");
    assert_eq!(result, 0x08);
  }

  #[test]
  fn test_fork() {
    let mut stream = Stream::open("sample/sample.arw").expect("Failed to open");
    stream.skip(4).expect("Failed to skip");
    assert_eq!(stream.position().expect("Failed to get pos"), 4);
    stream.fork(|stream| {
      stream.seek(0x1000)?;
      assert_eq!(stream.position().expect("Failed to get pos"), 0x1000);
      Ok(())
    }).expect("Failed to run test");
    assert_eq!(stream.position().expect("Failed to get pos"), 4);
  }
}
