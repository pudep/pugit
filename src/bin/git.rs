use pugit::git::{Git, HeadState};

fn main() -> anyhow::Result<()>{
  let git = Git::new("~/.config/nvim/")?;
  let repo = git.repo;

  let head_state = Git::get_head_state(&repo);
  match &head_state {
    HeadState::Branch(name) => println!("Branch: {name}"),
    HeadState::Detached(oid) => println!("Detached Head, Oid: {oid}"),
    HeadState::Error(e) => println!("Error: {e}"),
    HeadState::Unborn => println!("Branch is unborn")
  }
  Ok(())
}
