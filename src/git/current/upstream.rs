use git2::{Branch, Commit, Repository};

use crate::git::{Git, current::local::Local};

#[allow(dead_code)]
pub enum Upstream<'repo> {
  Branch(Branch<'repo>),
  Commit(Commit<'repo>),
  Error(String),
  None,
}

impl Git {
  pub fn get_oid_current<'repo>(
    repo: &'repo Repository,
    current: &Local,
  ) -> anyhow::Result<Upstream<'repo>, anyhow::Error> {
    match current {
      Local::Branch(b) => match b.upstream() {
        Ok(localbranch) => Ok(Upstream::Commit(
          repo.find_commit(localbranch.get().target().unwrap())?,
        )),
        Err(e) => Ok(Upstream::Error(e.to_string())),
      },
      Local::Error(error) => Ok(Upstream::Error(error.to_string())),
    }
  }

  /// This will return the `refs/remote/upstream` for current local branch.
  /// E.g : Local branch -> `main`
  ///       Upstream -> `origin/main`
  /// branch_name.upstream()? tells which Remote branch do Local one is tracking?
  pub fn get_current_upstream<'repo>(current: &Local<'repo>) -> Upstream<'repo> {
    match &current {
      Local::Branch(local_branch) => match local_branch.upstream() {
        Ok(k) => Upstream::Branch(k),
        Err(e) => Upstream::Error(e.to_string()),
      },
      _ => Upstream::None,
    }
  }
}
