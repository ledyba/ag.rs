mod arw;
use arw::ArwDecoder;

pub struct Image {
  width: usize,
  height: usize,
  data: Vec<u8>,
}

trait RawDecoder {
  fn is_acceptable(&self) -> bool;
  fn decode(&self) -> Result<Image, anyhow::Error>;
}
