use std::path::Path;
use crate::tiff;
use crate::raw::{ArwDecoder, RawDecoder};

pub fn render(input_path: impl AsRef<Path>, output_path: impl AsRef<Path>) -> anyhow::Result<()>{
  let mut stream = tiff::ByteStream::open(input_path)?;
  let mut parser = tiff::Parser::new(&mut stream);
  let tiff = parser.parse()?;
  tiff.inspect();
  //tiff::dumper::Dumper::new(&mut stream, &tiff).dump()?;
  let mut decoder = ArwDecoder::new(&mut stream, &tiff);
  if !decoder.is_acceptable() {
    return Err(anyhow::Error::msg("This file is not ARW!"));
  }
  let img = decoder.decode()?;
  img.save_to_file(output_path, false)?;

  Ok(())
}
