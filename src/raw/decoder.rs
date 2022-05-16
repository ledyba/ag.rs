mod arw;
pub use arw::ArwDecoder;
pub use crate::img::Image;

pub trait RawDecoder {
  fn is_acceptable(&self) -> bool;
  fn decode(&mut self) -> Result<Image, anyhow::Error>;
}
