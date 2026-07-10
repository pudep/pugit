mod action;
mod data;
mod cmd;
mod keys;
mod state;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
  cmd::parser();
  let mut app = crate::state::tabs::App::new();
  app.run()?;
  Ok(())
}
