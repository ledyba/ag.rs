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
      let left_to_load = bits - loaded_bits;
      let load_bits = min(left_to_load, self.buff_left);
      r = (r << load_bits) | (self.buff & (0xff >> (32-load_bits)));
      self.buff <<= load_bits;
      loaded_bits += load_bits;
      self.buff_left -= load_bits;
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
