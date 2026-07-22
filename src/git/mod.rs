use git2::Repository;
use crate::git::head::Head;
pub mod current;
pub mod head;
pub mod local;
pub mod refresh;
pub mod repo_state;
pub mod string_to_path;
pub mod upstream;

#[allow(dead_code)]
pub struct Git {
  pub repo: Repository,
  pub head: Head,
}

#[allow(dead_code)]
impl Git {
  pub fn new(path: &str) -> anyhow::Result<Self> {
    let repo = Repository::open(Git::string_to_path(path)?)?;
    let head = Git::get_current_head(&repo)?;
    Ok(Self { repo, head })
  }
}
