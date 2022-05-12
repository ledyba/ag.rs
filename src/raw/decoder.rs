mod arw;
pub use arw::ArwDecoder;

pub struct Image {
  width: usize,
  height: usize,
  data: Vec<u16>,
}

impl Image {
  pub fn new(width: usize, height: usize) -> Self {
    Self {
      width,
      height,
      data: vec![0; width * height],
    }
  }
  pub fn width(&self) -> usize {
    self.width
  }
  pub fn height(&self) -> usize {
    self.height
  }
  pub fn data(&self) -> &Vec<u16> {
    &self.data
  }
  fn calc_idx(&self, x: usize, y: usize) -> usize {
    self.width * y + x
  }
  pub fn set(&mut self, x: usize, y: usize, v: u16) {
    let idx = self.calc_idx(x,y);
    self.data[idx] = v;
  }
}

pub trait RawDecoder {
  fn is_acceptable(&self) -> bool;
  fn decode(&mut self) -> Result<Image, anyhow::Error>;
}
