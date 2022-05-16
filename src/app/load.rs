use crate::tiff;
use log::{error, info};
use crate::raw::{ArwDecoder, RawDecoder};

pub fn load(path: &str) -> anyhow::Result<()> {
  let mut stream = tiff::ByteStream::open(path)?;
  let mut parser = tiff::Parser::new(&mut stream);
  let tiff = parser.parse()?;
  tiff.inspect();
  tiff::dumper::Dumper::new(&mut stream, &tiff).dump()?;
  let mut decoder = ArwDecoder::new(&mut stream, &tiff);
  if !decoder.is_acceptable() {
    return Err(anyhow::Error::msg("This file is not ARW!"));
  }
  let img = decoder.decode()?;
  Ok(())
}
