use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use png::BitDepth;

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
  pub fn save_to_file(&self, path: impl AsRef<Path>) -> anyhow::Result<()> {
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    self.save(writer)
  }
  pub fn save<F: std::io::Write>(&self, writer: BufWriter<F>) -> anyhow::Result<()> {
    let mut encoder = png::Encoder::new(writer, self.width as u32, self.height as u32); // Width is 2 pixels and height is 1.
    //encoder.set_depth(BitDepth::Sixteen);
    encoder.set_depth(BitDepth::Eight);
    encoder.set_color(png::ColorType::Grayscale);
    let mut writer = encoder.write_header()?;
    let pixels = self.create_pixels();
    writer.write_image_data(&pixels).map_err(|it| anyhow::Error::from(it))
  }
  fn create_pixels(&self) -> Vec<u8> {
    let mut dst = Vec::<u8>::new();
    for pix in &self.data {
      /*
      let pix = pix << 4;
      dst.push((pix >> 8) as u8);
      dst.push((pix & 0xff) as u8);
      */
      dst.push(((pix >> 4) & 0xff) as u8);
    }
    dst
  }
}
