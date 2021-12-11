mod tiff;

fn main() -> anyhow::Result<()> {
  env_logger::init();
  let mut parser = tiff::Parser::open("./sample/sample.arw")?;
  let _tiff = parser.parse()?;
  Ok(())
}
