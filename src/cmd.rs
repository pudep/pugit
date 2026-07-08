use clap::Parser;
#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
  init: Option<String>,
  config: Option<String>
}

pub fn parser(){
  let cli = Cli::parse();
  if let Some(cfg) = cli.config {
    println!("{cfg}")
  }
}
