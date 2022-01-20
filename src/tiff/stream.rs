use std::io::{Read, Seek, SeekFrom};
use byteordered::{Endian, Endianness};
use byteordered::byteorder::ReadBytesExt;
use super::data_type::{UnsignedRational, SignedRational};

pub struct Stream {
  endian: Endianness,
  file: std::fs::File,
}

impl Stream {
  pub fn open(path: &str) -> std::io::Result<Stream> {
    let mut file = std::fs::File::open(path)?;
    let endian = {
      let mut header: [u8; 2] = [0, 0];
      file.read_exact(&mut header)?;
      if header == [0x4D, 0x4D] {
        Endianness::Big
      } else if header == [0x49, 0x49] {
        Endianness::Little
      } else {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Not a TIFF file."));
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
  pub fn fetch_u8(&mut self, offset: u64) -> std::io::Result<u8> {
    self.warp(offset, |s| s.read_u8())
  }
  pub fn read_vec_u8(&mut self, n: usize) -> std::io::Result<Vec<u8>> {
    let mut buff = Vec::<u8>::new();
    buff.resize(n, 0);
    self.file.read_exact(&mut buff)?;
    Ok(buff)
  }
  pub fn fetch_vec_u8(&mut self, offset: u64, n: usize) -> std::io::Result<Vec<u8>> {
    self.warp(offset, |s| s.read_vec_u8(n))
  }

  /* ASCII */
  pub fn read_ascii(&mut self, n: usize) -> std::io::Result<String> {
    let buff = self.read_vec_u8(n - 1)?;
    Ok(String::from_utf8_lossy(&buff).to_string())
  }
  pub fn fetch_ascii(&mut self, offset: u64, n: usize) -> std::io::Result<String> {
    self.warp(offset, |s| s.read_ascii(n))
  }

  /* i8 */
  pub fn read_i8(&mut self) -> std::io::Result<i8> {
    self.file.read_i8()
  }
  pub fn fetch_i8(&mut self, offset: u64) -> std::io::Result<i8> {
    self.warp(offset, |s| s.read_i8())
  }
  pub fn read_vec_i8(&mut self, n: usize) -> std::io::Result<Vec<i8>> {
    let mut buff = Vec::<i8>::new();
    buff.resize(n, 0);
    self.file.read_i8_into(&mut buff)?;
    Ok(buff)
  }
  pub fn fetch_vec_i8(&mut self, offset: u64, n: usize) -> std::io::Result<Vec<i8>> {
    self.warp(offset, |s| s.read_vec_i8(n))
  }

  /* u16 */
  pub fn read_u16(&mut self) -> std::io::Result<u16> {
    self.endian.read_u16(&mut self.file)
  }
  pub fn fetch_u16(&mut self, offset: u64) -> std::io::Result<u16> {
    self.warp(offset, |s| s.endian.read_u16(&mut s.file))
  }
  pub fn read_vec_u16(&mut self, n: usize) -> std::io::Result<Vec<u16>> {
    let mut buff: Vec<u16> = Vec::new();
    buff.resize(n, 0);
    self.endian.read_u16_into(&mut self.file, &mut buff)?;
    Ok(buff)
  }
  pub fn fetch_vec_u16(&mut self, offset: u64, n: usize) -> std::io::Result<Vec<u16>> {
    self.warp(offset, |s| s.read_vec_u16(n))
  }

  /* s16 */
  pub fn read_i16(&mut self) -> std::io::Result<i16> {
    self.endian.read_i16(&mut self.file)
  }
  pub fn fetch_i16(&mut self, offset: u64) -> std::io::Result<i16> {
    self.warp(offset, |s| s.read_i16())
  }
  pub fn read_vec_i16(&mut self, n: usize) -> std::io::Result<Vec<i16>> {
    let mut buff: Vec<i16> = Vec::new();
    buff.resize(n, 0);
    self.endian.read_i16_into(&mut self.file, &mut buff)?;
    Ok(buff)
  }
  pub fn fetch_vec_i16(&mut self, offset: u64, n: usize) -> std::io::Result<Vec<i16>> {
    self.warp(offset, |s| s.read_vec_i16(n))
  }

  /* u32 */
  pub fn read_u32(&mut self) -> std::io::Result<u32> {
    self.endian.read_u32(&mut self.file)
  }
  pub fn fetch_u32(&mut self, offset: u64) -> std::io::Result<u32> {
    self.warp(offset, |s| s.read_u32())
  }
  pub fn read_vec_u32(&mut self, n: usize) -> std::io::Result<Vec<u32>> {
    let mut buff: Vec<u32> = Vec::new();
    buff.resize(n, 0);
    self.endian.read_u32_into(&mut self.file, &mut buff)?;
    Ok(buff)
  }
  pub fn fetch_vec_u32(&mut self, offset: u64, n: usize) -> std::io::Result<Vec<u32>> {
    self.warp(offset, |s| s.read_vec_u32(n))
  }

  /* s32 */
  pub fn read_i32(&mut self) -> std::io::Result<i32> {
    self.endian.read_i32(&mut self.file)
  }
  pub fn fetch_i32(&mut self, offset: u64) -> std::io::Result<i32> {
    self.warp(offset, |s| s.read_i32())
  }
  pub fn read_vec_i32(&mut self, n: usize) -> std::io::Result<Vec<i32>> {
    let mut buff: Vec<i32> = Vec::new();
    buff.resize(n, 0);
    self.endian.read_i32_into(&mut self.file, &mut buff)?;
    Ok(buff)
  }
  pub fn fetch_vec_i32(&mut self, offset: u64, n: usize) -> std::io::Result<Vec<i32>> {
    self.warp(offset, |s| s.read_vec_i32(n))
  }

  /* f32 */
  pub fn read_f32(&mut self) -> std::io::Result<f32> {
    self.endian.read_f32(&mut self.file)
  }
  pub fn fetch_f32(&mut self, offset: u64) -> std::io::Result<f32> {
    self.warp(offset, |s| s.read_f32())
  }
  pub fn read_vec_f32(&mut self, n: usize) -> std::io::Result<Vec<f32>> {
    let mut buff: Vec<f32> = Vec::new();
    buff.resize(n, 0.0);
    self.endian.read_f32_into(&mut self.file, &mut buff)?;
    Ok(buff)
  }
  pub fn fetch_vec_f32(&mut self, offset: u64, n: usize) -> std::io::Result<Vec<f32>> {
    self.warp(offset, |s| s.read_vec_f32(n))
  }

  /* f64 */
  pub fn read_f64(&mut self) -> std::io::Result<f64> {
    self.endian.read_f64(&mut self.file)
  }
  pub fn fetch_f64(&mut self, offset: u64) -> std::io::Result<f64> {
    self.warp(offset, |s| s.read_f64())
  }
  pub fn read_vec_f64(&mut self, n: usize) -> std::io::Result<Vec<f64>> {
    let mut buff: Vec<f64> = Vec::new();
    buff.resize(n, 0.0);
    self.endian.read_f64_into(&mut self.file, &mut buff)?;
    Ok(buff)
  }
  pub fn fetch_vec_f64(&mut self, offset: u64, n: usize) -> std::io::Result<Vec<f64>> {
    self.warp(offset, |s| s.read_vec_f64(n))
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
  pub fn fetch_unsigned_rational(&mut self, offset: u64) -> std::io::Result<UnsignedRational> {
    self.warp(offset, |s| s.read_unsigned_rational())
  }
  pub fn read_vec_unsigned_rational(&mut self, n: usize) -> std::io::Result<Vec<UnsignedRational>> {
    let buff = self.read_vec_u32(n * 2)?;
    let values: Vec<UnsignedRational> = buff.chunks_exact(2).map(|v| UnsignedRational {
      numerator: v[0],
      denominator: v[1],
    }).collect();
    Ok(values)
  }
  pub fn fetch_unsigned_rationals(&mut self, offset: u64, n: usize) -> std::io::Result<Vec<UnsignedRational>> {
    self.warp(offset, |s| s.read_vec_unsigned_rational(n))
  }

  /* SRational */
  pub fn read_signed_rational(&mut self) -> std::io::Result<SignedRational> {
    let mut buff: [i32; 2] = [0, 0];
    self.endian.read_i32_into(&mut self.file, &mut buff)?;
    Ok(SignedRational {
      numerator: buff[0],
      denominator: buff[1],
    })
  }
  pub fn fetch_signed_rational(&mut self, offset: u64) -> std::io::Result<SignedRational> {
    self.warp(offset, |s| s.read_signed_rational())
  }
  pub fn read_vec_signed_rational(&mut self, n: usize) -> std::io::Result<Vec<SignedRational>> {
    let buff = self.read_vec_i32(n * 2)?;
    let values: Vec<SignedRational> = buff.chunks_exact(2).map(|v| SignedRational {
      numerator: v[0],
      denominator: v[1],
    }).collect();
    Ok(values)
  }
  pub fn fetch_signed_rationals(&mut self, offset: u64, n: usize) -> std::io::Result<Vec<SignedRational>> {
    self.warp(offset, |s| s.read_vec_signed_rational(n))
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

  fn warp<'s, Fn, T>(&'s mut self, offset: u64, f: Fn) -> std::io::Result<T>
    where
    // https://doc.rust-lang.org/reference/trait-bounds.html#higher-ranked-trait-bounds
    for<'f> Fn: FnOnce(&'f mut Self) -> std::io::Result<T>,
  {
    self.fork(|stream| {
      stream.seek(offset)?;
      f(stream)
    })
  }

  fn fork<'s, Fn, T>(&'s mut self, f: Fn) -> std::io::Result<T>
    where
    for <'f> Fn: FnOnce(&'f mut Self) -> std::io::Result<T>,
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
