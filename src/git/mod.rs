use anyhow::Context;
use git2::{Branch, Commit, ErrorCode, Reference, Repository};
use std::path::{Path, PathBuf};

#[allow(dead_code)]
/// Used to match the state of HEAD
pub enum HeadState<'repo> {
  /// Valid only if head is a branch
  Refrence(Reference<'repo>),
  /// Valid only if head is not a branch but available
  Detached(Commit<'repo>),
  /// Serious error to display
  Error(String),
  /// State when Branch is unborn
  Unborn,
}

#[allow(dead_code)]
/// The Remote commit containg enum
pub enum Upstream<'repo> {
  Commit(Commit<'repo>),
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
  pub fn get_current_head<'repo>(
    repo: &'repo Repository,
  ) -> anyhow::Result<HeadState<'repo>, anyhow::Error> {
    match repo.head() {
      // A head (latest commit) can point either to a Branch say Main or to a commit(only if is detached) so :
      Ok(head) => {
        if head.is_branch() {
          Ok(HeadState::Refrence(head))
        } else {
          match head.target() {
            Some(oid) => Ok(HeadState::Detached(repo.find_commit(oid)?)),
            None => Ok(HeadState::Error(
              "Detached HEAD but points to no Commit.".to_string(),
            )),
          }
        }
      }
      // This is done to tell user that the Branch is unborn.
      Err(e) if e.code() == ErrorCode::UnbornBranch => Ok(HeadState::Unborn),

      // This displays serious to resolve errors.
      Err(e) => Ok(HeadState::Error(e.to_string())),
    }
  }

  /// Validity of `Branch<'_>` depends on `Repository`.
  /// Vulnerable to staling.
  /// Status: Unchecked by `ipude`.
  pub fn get_current_local_branch<'repo>(
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

  /// Get latest oid of the local branch which has been pushed to the underlying remote.
  pub fn get_oid_current<'repo>(
    repo: &'repo Repository,
    current: &Current,
  ) -> anyhow::Result<Upstream<'repo>, anyhow::Error> {
    match current {
      Current::LocalBranch(b) => match b.upstream() {
        Ok(localbranch) => Ok(Upstream::Commit(
          repo.find_commit(localbranch.get().target().unwrap())?,
        )),
        Err(e) => Ok(Upstream::Error(e.to_string())),
      },
      Current::Error(error) => Ok(Upstream::Error(error.to_string())),
    }
  }

  /// This will return the `refs/remote/upstream` for current local branch.
  /// E.g : Local branch -> `main`
  ///       Upstream -> `origin/main`
  pub fn get_current_upstream(current: &Current) -> anyhow::Result<String> {
    match &current {
      Current::LocalBranch(local_branch) => Ok(
        local_branch
          .upstream()?
          .name()?
          .unwrap_or("<No Remote>")
          .to_string(),
      ),
      _ => Ok("Nil".to_string()),
    }
  }
}
