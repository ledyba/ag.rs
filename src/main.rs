use clap::Arg;
use log::LevelFilter;

mod app;
mod tiff;
mod raw;
mod stream;
mod img;

fn main() -> anyhow::Result<()> {
  let mut log_builder = env_logger::Builder::from_default_env();
  let app = clap::App::new("ag")
    .author("Kaede Fujisaki")
    .about("ARW Parser and Image generator")
    .setting(clap::AppSettings::ArgRequiredElseHelp)
    .arg(Arg::new("verbose")
      .long("verbose")
      .short('v')
      .required(false)
      .takes_value(false)
      .help("Show verbose message"))
    .subcommand(clap::App::new("load")
      .arg(Arg::new("input.arw")
        .help("File path to load")
        .index(1)
        .takes_value(true)
        .required(true))
      .arg(Arg::new("output.png")
        .help("File path to save")
        .index(2)
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
        if let (Some(input), Some(output)) = (m.value_of("input.arw"), m.value_of("output.png")) {
          return app::load(input, output);
        }
      }
      cmd => {
        return Err(anyhow::Error::msg(format!("Unknown command: {}", cmd)));
      }
    }
  }
  // Nothing to do!
  return Err(anyhow::Error::msg("Please specify a subcommand to do."));
}
