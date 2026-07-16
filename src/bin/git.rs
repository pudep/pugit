use std::process::id;

use pugit::git::{Git, Upstream};

fn main() -> anyhow::Result<()> {
  let git = Git::new("~/impl/rust/pugit/")?;
  let repo = git.repo;

  let head_state = Git::get_current_head(&repo)?;
  let local_branch = Git::get_current_local_branch(&repo, &head_state);
  let local_branch_oid = Git::get_oid_current(&repo, &local_branch)?;
  match local_branch_oid {
    Upstream::Commit(commit) => println!("Oid: {:?}\nMessage: {}", commit, commit.message()?),
    Upstream::Error(e) => println!("{e}"),
  }

  println!("{}", Git::get_current_upstream(&local_branch)?);
  Ok(())
}
