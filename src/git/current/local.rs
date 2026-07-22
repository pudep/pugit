use git2::{Branch, RepositoryState};

use crate::git::Git;

#[allow(dead_code)]
pub enum Local<'repo> {
  Branch(Branch<'repo>),
  Error(String),
}

impl Git {
  pub fn get_current_local_branch<'repo>(&mut self, repo: &'repo RepositoryState) {
    // if self.check.head.is_head {}
    // match repo.find_branch(string.shorthand().unwrap(), git2::BranchType::Local) {
    //   Ok(b) => Local::Branch(b),
    //   Err(e) => Local::Error(e.to_string()),
    // }
    // }
    // Local::Error("The Head::Refrence(refrence) was likely invalid".to_string()),
  }
}
