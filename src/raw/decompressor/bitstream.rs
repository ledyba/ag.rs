use crate::tiff::ByteStream;

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
    todo!()
  }
}

#[cfg(test)]
mod test {
  #[test]
  fn basic() {

  }
}
