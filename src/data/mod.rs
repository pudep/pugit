use anyhow::Context;
use git2::{Branch, ErrorCode, Repository, RepositoryState};
use std::{
  error::Error,
  path::{Path, PathBuf},
  result,
};

#[derive(Debug)]
enum HeadState {
  Detached(String),
  Branch(String),
  Error(String),
  Unborn,
}

enum Upstream {
  Oid(String),
  Error(String),
}

enum LocalBranch<'repo>{
  Branch(Branch<'repo>),
  Error(String),
}

#[allow(dead_code)]
pub struct GitHealth {
  pub branch: String,
  pub detached: bool,
  pub staged: usize,
  pub unstaged: usize,
  pub untracked: usize,
  pub conflicts: usize,
  pub ahead: usize,
  pub behind: usize,
  pub state: RepositoryState,
  pub stash_count: usize,
}

#[allow(dead_code)]
impl GitHealth {
  /// Parses the repo path string to Path by expanding
  /// all the enviromental variables.
  /// Status : Accurate and Tested.
  fn repo_path_parser(path_string: &str) -> anyhow::Result<PathBuf> {
    let expanded = shellexpand::full(path_string)
      .with_context(|| format!("failed to expand path: `{path_string}`"))?;

    let canonical = Path::new(expanded.as_ref())
      .canonicalize()
      .with_context(|| format!("The expanded path do not exists or is inaccessible : `{expanded}` probably the `{path_string}` is wrong."))?;
    Ok(canonical)
  }
  fn get_repo(path: PathBuf) -> Result<Repository, Box<dyn Error>> {
    let repo = Repository::open(path)?;
    Ok(repo)
  }

  /// This method returns enum `HeadState` and will fail gracefully.
  /// Check enum `HeadState` to know what it returns.
  /// Status : Accurate and Tested.
  fn head_state(repo: &Repository) -> result::Result<HeadState, Box<dyn Error>> {
    let head = repo.head();
    let head_state = match head {
      Ok(head) => {
        // detached head points to a commit (oid)
        if repo.head_detached()? {
          HeadState::Detached(head.target().unwrap().to_string())
        }
        // attached head points to a branch
        // if there is a branch unwrap the name or else return "unkown"
        else {
          HeadState::Branch(head.shorthand().unwrap_or("unkown").to_string())
        }
      }
      // To handle the unborn branch case
      Err(e) if e.code() == ErrorCode::UnbornBranch => HeadState::Unborn,

      // To display a serious error
      Err(e) => HeadState::Error(e.to_string()),
    };
    Ok(head_state)
  }

  /// This method will only live until repo is valid
  /// Since `LocalBranch` is derived from repo then if I hold the repo in memory then likely I can get access to this data also whenever I want.
  /// Make sure `LocalBranch` must not be stored directly as a static data.
  /// Status : Not tested tested yet (theorretical only)
  fn get_local_branch<'repo>(repo: &'repo Repository, head_state: HeadState) -> LocalBranch<'repo> {
    match head_state {
      HeadState::Branch(name) => {
        match repo.find_branch(&name, git2::BranchType::Local) {
          Ok(b) => LocalBranch::Branch(b),
          Err(e) => LocalBranch::Error(e.to_string()),
        }
      }
      _ => LocalBranch::Error("No such LocalBranch found for HeadState::Branch(name)".to_string())
    }
  }

  fn get_remote_oid() {
    let upstream = match local_branch.upstream() {
      Ok(local_branch) => Upstream::Oid(local_branch.get().target().unwrap().to_string()),
      Err(e) => Upstream::Error(e.to_string()),
    };
  }
}
