use anyhow::Context;
use git2::{Branch, ErrorCode, Oid, Reference, Repository};
use std::path::{Path, PathBuf};

#[allow(dead_code)]
/// Used to match the state of HEAD
pub enum HeadState<'repo> {
  /// Valid only if head is a branch
  Refrence(Reference<'repo>),
  /// Valid only if head is not a branch but available
  Detached(Oid),
  /// Serious error to display
  Error(String),
  /// State when Branch is unborn
  Unborn,
}

#[allow(dead_code)]
pub enum Remote {
  Oid(String),
  Error(String),
}

#[allow(dead_code)]
/// Stores Current (mut) data from Repository
/// Status : Accurate & Tested by `ipude`
pub enum Current<'repo> {
  LocalBranch(Branch<'repo>),
  Error(String),
}

#[allow(dead_code)]
/// The core Git structure that holds lifelong and expensive to recalculate variables.
pub struct Git {
  pub repo: Repository,
}

#[allow(dead_code)]
impl Git {
  pub fn new(path: &str) -> anyhow::Result<Self> {
    Ok(Self {
      repo: Repository::open(Git::string_to_path(path)?)?,
    })
  }
}

#[allow(dead_code)]
impl Git {
  /// Parses String into PathBuf via crate:  `Shellexpand`.
  /// Status : Accurate and Tested by `ipude`.
  fn string_to_path(path_string: &str) -> anyhow::Result<PathBuf> {
    let expanded = shellexpand::full(path_string)
      .with_context(|| format!("failed to expand path: `{path_string}`"))?;

    let canonical = Path::new(expanded.as_ref())
      .canonicalize()
      .with_context(|| format!("The expanded path do not exists or is inaccessible : `{expanded}` probably the `{path_string}` is wrong."))?;
    Ok(canonical)
  }

  /// Retuns enum `HeadState`.
  /// Status: Accurate and Tested by `ipude`.
  pub fn get_head_state<'repo>(repo: &'repo Repository) -> HeadState<'repo> {
    match repo.head() {
      // A head (latest commit) can point either to a Branch say Main or to a commit(only if is detached) so :
      Ok(head) => {
        if head.is_branch() {
          HeadState::Refrence(head)
        } else {
          match head.target() {
            Some(oid) => HeadState::Detached(oid),
            None => HeadState::Error("Detached HEAD but points to no Commit.".to_string()),
          }
        }
      }
      // This is done to tell user that the Branch is unborn.
      Err(e) if e.code() == ErrorCode::UnbornBranch => HeadState::Unborn,

      // This displays serious to resolve errors.
      Err(e) => HeadState::Error(e.to_string()),
    }
  }

  /// Validity of `Branch<'_>` depends on `Repository`.
  /// Vulnerable to staling.
  /// Status: Unchecked by `ipude`.
  pub fn get_local_branch<'repo>(
    repo: &'repo Repository,
    head_state: &HeadState,
  ) -> Current<'repo> {
    match head_state {
      HeadState::Refrence(refrence) => {
        match repo.find_branch(refrence.shorthand().unwrap(), git2::BranchType::Local) {
          Ok(b) => Current::LocalBranch(b),
          Err(e) => Current::Error(e.to_string()),
        }
      }
      _ => Current::Error("The HeadState::Refrence(refrence) was likely invalid".to_string()),
    }
  }

  // fn get_remote_oid() {
  //   let upstream = match local_branch.upstream() {
  //     Ok(local_branch) => Remote::Oid(local_branch.get().target().unwrap().to_string()),
  //     Err(e) => Remote::Error(e.to_string()),
  //   };
  // }
}
