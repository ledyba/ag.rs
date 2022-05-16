use std::collections::HashMap;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use log::info;
use png::BitDepth;
use png::Compression::Default;
use crate::tiff::CFAPattern;

pub struct RawImage {
  width: usize,
  height: usize,
  data: Vec<u16>,
  cfa_pattern: Vec<CFAPattern>,
  cfa_dim: (usize, usize),
}

impl RawImage {
  pub fn new(
    width: usize,
    height: usize,
    cfa_pattern: Vec<CFAPattern>,
    cfa_dim: (usize, usize),
  ) -> Self {
    Self {
      width,
      height,
      data: vec![0; width * height],
      cfa_pattern,
      cfa_dim,
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
  pub fn get(&self, x: usize, y: usize) -> (u16, u16, u16) {
    let idx = self.calc_idx(x,y);
    let color = self.data[idx] << 4;
    let row = y % self.cfa_dim.1;
    let col = x % self.cfa_dim.0;
    match self.cfa_pattern[row * self.cfa_dim.0 + col] {
      CFAPattern::R => (color, 0, 0),
      CFAPattern::G => (0, color, 0),
      CFAPattern::B => (0, 0, color),
      CFAPattern::Unknown(_) => (color, color, color),
    }
  }
  pub fn get_mixed(&self, x: usize, y: usize) -> (u16, u16, u16) {
    // FIXME: better denoising
    let origin_x = x - (x % self.cfa_dim.0);
    let origin_y = y - (y % self.cfa_dim.1);
    let mut colors = [Vec::<u16>::new(), Vec::<u16>::new(), Vec::<u16>::new()];
    for dy in 0..self.cfa_dim.1 {
      for dx in 0..self.cfa_dim.0 {
        let idx = self.calc_idx(origin_x + dx, origin_y + dy);
        let color = self.data[idx] << 4;
        match self.cfa_pattern[dy * self.cfa_dim.0 + dx] {
          CFAPattern::R => colors[0].push(color),
          CFAPattern::G => colors[1].push(color),
          CFAPattern::B => colors[2].push(color),
          CFAPattern::Unknown(_) => {
            colors[0].push(color);
            colors[1].push(color);
            colors[2].push(color);
          },
        };
      }
    }
    let r = colors[0].iter().map(|it| *it as u32).sum::<u32>() / colors[0].len() as u32;
    let g = colors[1].iter().map(|it| *it as u32).sum::<u32>() / colors[1].len() as u32;
    let b = colors[2].iter().map(|it| *it as u32).sum::<u32>() / colors[2].len() as u32;
    (r as u16, g as u16, b as u16)
  }
  pub fn save_to_file(&self, path: impl AsRef<Path>, high_bits: bool) -> anyhow::Result<()> {
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    self.save(writer, high_bits)
  }
  pub fn save<F: std::io::Write>(&self, writer: BufWriter<F>, high_bits: bool) -> anyhow::Result<()> {
    let mut encoder = png::Encoder::new(writer, self.width as u32, self.height as u32); // Width is 2 pixels and height is 1.
    encoder.set_color(png::ColorType::Rgb);
    if high_bits {
      encoder.set_depth(BitDepth::Sixteen);
    } else {
      encoder.set_depth(BitDepth::Eight);
    }
    let mut writer = encoder.write_header()?;
    let pixels = self.create_pixels(high_bits);
    writer.write_image_data(&pixels).map_err(|it| anyhow::Error::from(it))
  }
  fn create_pixels(&self, high_bits: bool) -> Vec<u8> {
    let mut buff = Vec::<u8>::new();
    for y in 0..self.height {
      for x in 0..self.width {
        let (r, g, b) = self.get_mixed(x, y);
        if high_bits {
          buff.push((r >> 8) as u8);
          buff.push((r & 0xff) as u8);
          buff.push((g >> 8) as u8);
          buff.push((g & 0xff) as u8);
          buff.push((b >> 8) as u8);
          buff.push((b & 0xff) as u8);
        } else {
          buff.push((r >> 8) as u8);
          buff.push((g >> 8) as u8);
          buff.push((b >> 8) as u8);
        }
      }
    }
    buff
  }
}
