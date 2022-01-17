use clap::{Arg, SubCommand};
use log::LevelFilter;

mod app;
mod tiff;

fn main() -> anyhow::Result<()> {
  let mut log_builder = env_logger::Builder::from_default_env();
  let app = clap::App::new("ag")
    .author("Kaede Fujisaki")
    .about("ARW Parser and Image generator")
    .arg(Arg::with_name("verbose")
      .long("verbose")
      .short("v")
      .required(false)
      .takes_value(false)
      .help("Show verbose message"))
    .subcommand(SubCommand::with_name("load")
      .arg(Arg::with_name("filename.arw")
        .help("File path to load")
        .index(1)
        .multiple(false)
        .takes_value(true)
        .required(true)));
  let m = app.get_matches();
  if m.is_present("verbose") {
    log_builder.filter_level(LevelFilter::Debug);
  }
  log_builder.init();

  if let Some(command_name) = m.subcommand_name() {
    match command_name {
      "load" => {
        let m = m.subcommand_matches("load").unwrap();
        if let Some(path) = m.value_of("filename.arw") {
          return app::load(path);
        }
      }
      _ => {}
    }
  }
  // Nothing to do!
  eprintln!("{}", m.usage());
  return Err(anyhow::Error::msg("Please specify a subcommand to do."));
}
