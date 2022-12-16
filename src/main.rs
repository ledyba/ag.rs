use clap::{Arg, ArgAction, value_parser};

mod app;
mod tiff;
mod raw;
mod stream;
mod img;

fn app() -> clap::Command {
  clap::Command::new("ag")
      .author("Kaede Fujisaki")
      .about("ARW Parser and Image generator")
      .arg(Arg::new("verbose")
          .long("verbose")
          .short('v')
          .required(false)
          .action(ArgAction::Count)
          .value_parser(value_parser!(u8))
          .help("Show verbose message"))
      .subcommand(clap::Command::new("load")
          .arg(Arg::new("input.arw")
              .help("File path to load")
              .index(1)
              .action(ArgAction::Set)
              .value_parser(value_parser!(String))
              .required(true))
          .arg(Arg::new("output.png")
              .help("File path to save")
              .index(2)
              .action(ArgAction::Set)
              .value_parser(value_parser!(String))
              .required(true)))
}

fn setup_logger(log_level: log::LevelFilter) -> Result<(), fern::InitError> {
  fern::Dispatch::new()
      .format(|out, message, record| {
        out.finish(format_args!(
          "{}[{}][{}] {}",
          chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
          record.target(),
          record.level(),
          message
        ))
      })
      .level(log_level)
      .chain(std::io::stdout())
      .chain(fern::log_file("output.log")?)
      .apply()?;
  Ok(())
}

fn main() -> anyhow::Result<()> {
  let app = app();
  let m = app.get_matches();
  let log_level = match m.get_one::<u8>("verbose") {
    None | Some(0) => log::LevelFilter::Info,
    Some(1) => log::LevelFilter::Debug,
    _ => log::LevelFilter::Trace,
  };
  setup_logger(log_level)?;

  let Some(command_name) = m.subcommand_name() else {
    // Nothing to do!
    return Err(anyhow::Error::msg("Please specify a subcommand to do."));
  };
  match command_name {
    "load" => {
      let m = m.subcommand_matches("load").unwrap();
      if let (Some(input), Some(output)) = (m.get_one::<String>("input.arw"), m.get_one::<String>("output.png")) {
        return app::load(input, output);
      }
    }
    cmd => {
      return Err(anyhow::Error::msg(format!("Unknown command: {}", cmd)));
    }
  }
  Ok(())
}
