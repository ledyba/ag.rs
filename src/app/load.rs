use crate::tiff;
use log::info;

pub fn load(path: &str) -> anyhow::Result<()> {
  let mut stream = tiff::Stream::open(path)?;
  let mut parser = tiff::Parser::new(&mut stream);
  let tiff = parser.parse()?;
  tiff.inspect();
  Ok(())
}
