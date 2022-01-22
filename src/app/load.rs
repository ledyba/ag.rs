use crate::tiff;
use log::info;

pub fn load(path: &str) -> anyhow::Result<()> {
  let mut parser = tiff::Parser::open(path)?;
  let tiff = parser.parse()?;
  tiff.inspect();
  Ok(())
}
