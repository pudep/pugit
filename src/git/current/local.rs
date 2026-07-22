use git2::Repository;

use crate::git::{Git, current::head::Head};

#[allow(dead_code)]
pub enum Local {
  Branch(String),
  Error(String),
  None,
}

impl Git {
  pub fn get_current_local_branch(head_ref: &Head, repo: &Repository) -> anyhow::Result<Local> {
    match &head_ref {
      Head::Refrence(name) => match repo.find_branch(name, git2::BranchType::Local) {
        Ok(branch) => Ok(Local::Branch(branch.name()?.unwrap().to_string())),
        Err(e) => Ok(Local::Error(e.to_string())),
      },
      _ => Ok(Local::None),
    }
  }
}
