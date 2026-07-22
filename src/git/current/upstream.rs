use git2::{Branch, Commit};

use crate::git::Git;

#[allow(dead_code)]
pub enum Upstream<'repo> {
  Branch(Branch<'repo>),
  Commit(Commit<'repo>),
  Error(String),
  None,
}

impl Git {
  // pub fn get_oid_current<'repo>(
  //   repo: &'repo Repository,
  //   current: &Local,
  // ) -> anyhow::Result<Upstream<'repo>, anyhow::Error> {
  //   match current {
  //     Local::Branch(b) => match b.upstream() {
  //       Ok(localbranch) => Ok(Upstream::Commit(
  //         repo.find_commit(localbranch.get().target().unwrap())?,
  //       )),
  //       Err(e) => Ok(Upstream::Error(e.to_string())),
  //     },
  //     Local::Error(error) => Ok(Upstream::Error(error.to_string())),
  //   }
  // }

  // pub fn get_current_upstream(current: &Local) -> Upstream {
  //   // match &current {
  //   //   Local::Branch(local_branch) => match local_branch.upstream() {
  //   //     Ok(k) => Upstream::Branch(k),
  //   //     Err(e) => Upstream::Error(e.to_string()),
  //   //   },
  //   //   _ => Upstream::None,
  //   // }
  // }
}
