mod arw;
pub use arw::ArwDecoder;
pub use crate::img::RawImage;

pub trait RawDecoder {
  fn is_acceptable(&self) -> bool;
  fn decode(&mut self) -> Result<RawImage, anyhow::Error>;
}
