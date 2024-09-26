pub(crate) mod cfg;
pub(crate) mod cli;
pub(crate) mod core;
pub(crate) mod error;
pub(crate) mod net;

use cli::colors::*;

#[tokio::main]
async fn run() -> error::Result<()> {
  Ok(())
}

fn main() {
  let _ = run()
    .inspect(|_| {
      println!("{} {}", "conserver".bold().green(), "has stopped running".green());
    })
    .map_err(|e| {
      eprintln!("{}: {}", "fatal".bold().red(), e.red());
      std::process::exit(1);
    });
}
