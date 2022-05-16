use std::cmp::min;
use crate::stream::ByteStream;

pub struct BitStream<'a> {
  stream: &'a mut ByteStream,
  buff: u32,
  buff_left: u8,
}

impl <'a> BitStream<'a> {
  pub fn new(stream: &'a mut ByteStream) -> Self {
    Self {
      stream,
      buff: 0,
      buff_left: 0,
    }
  }
  pub fn read_bits(&mut self, bits: u8) -> anyhow::Result<u32> {
    let mut r: u32 = 0;
    let mut loaded_bits = 0;
    while loaded_bits < bits {
      if self.buff_left == 0 {
        self.buff = self.stream.read_u32()?;
        self.buff_left = 32;
      }
      r = r | (self.buff & 1) << loaded_bits;
      self.buff >>= 1;
      loaded_bits += 1;
      self.buff_left -= 1;
    }
    Ok(r)
  }
}

#[cfg(test)]
mod test {
  #[test]
  fn basic() {

  }
}
