use pugit::git::Git;

fn main() -> anyhow::Result<()> {
  let git = Git::new("~/.config/nvim/")?;
  let repo = git.repo;

  let head_state = Git::get_head_state(&repo);
  let local_branch = Git::get_local_branch(&repo, &head_state);
  match &local_branch {
    pugit::git::Current::LocalBranch(branch_name) => {
      println!("Current Local Branch : {}", branch_name.name()?.unwrap())
    }
    pugit::git::Current::Error(e) => println!("Error (while searching for Current Local Branch ): {}", e),
  }
  Ok(())
}
