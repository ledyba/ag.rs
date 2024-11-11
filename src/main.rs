use clap::{Arg, ArgAction, value_parser};

mod app;
mod tiff;
mod raw;
mod stream;
mod img;

fn app() -> clap::Command {
  clap::Command::new("ag")
      .author("Kaede Fujisaki")
      .about("ARW Parser and Image renderer")
      .arg(Arg::new("verbose")
          .long("verbose")
          .short('v')
          .required(false)
          .action(ArgAction::Count)
          .value_parser(value_parser!(u8))
          .help("Show verbose message"))
      .subcommand(clap::Command::new("render")
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

fn main() -> anyhow::Result<()> {
  use tracing_subscriber::util::SubscriberInitExt;
  let app = app();
  let m = app.get_matches();
  let log_level = match m.get_one::<u8>("verbose") {
    None | Some(0) => tracing::Level::INFO,
    Some(1) => tracing::Level::DEBUG,
    _ => tracing::Level::TRACE,
  };
  tracing_subscriber::fmt()
    .with_timer(tracing_subscriber::fmt::time::ChronoLocal::new("%Y/%m/%d %H:%M:%S%.3f".to_string()))
    .with_max_level(log_level)
    .with_line_number(true)
    .with_file(true)
    .with_writer(std::io::stderr)
    .finish()
    .init();

  let Some(command_name) = m.subcommand_name() else {
    // Nothing to do!
    return Err(anyhow::Error::msg("Please specify a subcommand to do."));
  };
  match command_name {
    "render" => {
      let m = m.subcommand_matches("render").unwrap();
      let input = m.get_one::<String>("input.arw").expect("[BUG] No input!");
      let output = m.get_one::<String>("output.png").expect("[BUG] No output!");
      app::render(input, output)
    }
    cmd => {
      Err(anyhow::Error::msg(format!("Unknown command: {}", cmd)))
    }
  }
}
