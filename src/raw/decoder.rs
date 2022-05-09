mod arw;
pub use arw::ArwDecoder;

pub struct Image {
  width: usize,
  height: usize,
  data: Vec<u8>,
}

pub trait RawDecoder {
  fn is_acceptable(&self) -> bool;
  fn decode(&mut self) -> Result<Image, anyhow::Error>;
}
