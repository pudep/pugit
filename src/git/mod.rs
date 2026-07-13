use anyhow::Context;
use git2::{Branch, ErrorCode, Oid, Repository};
use std::path::{Path, PathBuf};

#[derive(Debug)]
/// This enum tells/holds one of the following listed values:
/// 1. Detached(type: String)
/// 2. Branch(type: String)
/// 3. Error(type: String)
/// 4. Unborn( type less I handle this case as a token/bool )
/// Method to get it : Git::get_head_state()
pub enum HeadState {
  Detached(Oid),
  Branch(String),
  Error(String),
  Unborn,
}

/// It holds Remote's oid or error
/// It fetches the oid if HeadState::Branch(name) contains some local branch name.
/// It uses the raw shorthand of the `Branch(name)` to fetch oid of the latest pushed commit.
///
/// State of use : May be unnecessary
pub enum Remote {
  Oid(String),
  Error(String),
}

/// This is only valid as long as the Repository is
/// Must not be stored into struct as it can stale if Branch is suddenly changed
/// Holds either of a `Branch<'repo>` or `Error: String`
pub enum LocalBranch<'repo> {
  Branch(Branch<'repo>),
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
  /// Parses String into PathBuf via Shellexpand
  /// Status : Accurate and Tested by `ipude`
  fn string_to_path(path_string: &str) -> anyhow::Result<PathBuf> {
    let expanded = shellexpand::full(path_string)
      .with_context(|| format!("failed to expand path: `{path_string}`"))?;

    let canonical = Path::new(expanded.as_ref())
      .canonicalize()
      .with_context(|| format!("The expanded path do not exists or is inaccessible : `{expanded}` probably the `{path_string}` is wrong."))?;
    Ok(canonical)
  }

  /// Retuns enum `HeadState`
  /// Status: Accurate and Tested by `ipude`
  pub fn get_head_state(repo: &Repository) -> HeadState {
    match repo.head() {
      // A head (latest commit) can point either to a Branch say Main or to a commit(only if is detached) so :
      Ok(head) => {
        if head.is_branch() {
          HeadState::Branch(head.shorthand().unwrap().to_string())
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

  /// This method will only live until repo is valid
  /// Since `LocalBranch` is derived from repo then if I hold the repo in memory then likely I can get access to this data also whenever I want.
  /// Make sure `LocalBranch` must not be stored directly as a static data.
  /// Status : Not tested tested yet (theorretical only)
  fn get_local_branch<'repo>(repo: &'repo Repository, head_state: HeadState) -> LocalBranch<'repo> {
    match head_state {
      HeadState::Branch(name) => match repo.find_branch(&name, git2::BranchType::Local) {
        Ok(b) => LocalBranch::Branch(b),
        Err(e) => LocalBranch::Error(e.to_string()),
      },
      _ => LocalBranch::Error("No such LocalBranch found for HeadState::Branch(name)".to_string()),
    }
  }

  // fn get_remote_oid() {
  //   let upstream = match local_branch.upstream() {
  //     Ok(local_branch) => Remote::Oid(local_branch.get().target().unwrap().to_string()),
  //     Err(e) => Remote::Error(e.to_string()),
  //   };
  // }
}
