use anyhow::Context;
use git2::{ErrorCode, Repository, RepositoryState};
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

  fn get_head_status(repo: &Repository) -> String {
    let head = match repo.head() {
      Ok(head) => head,
      Err(error) => return error.to_string(),
    };
    match repo.head_detached() {
      Ok(true) => match head.target() {
        Some(oid) => format!("detached @{}", oid),
        None => "detached (no target)".to_string(),
      },
      Ok(false) => head.shorthand().unwrap_or("unkown").to_string(),
      Err(error) => error.to_string(),
    }
  }

  fn get_branch(
    repo: &Repository,
    branch_name: &str,
  ) -> std::result::Result<std::string::String, Box<dyn Error>> {
    let branch = repo.find_branch(branch_name, git2::BranchType::Remote)?;
    match branch.name() {
      Ok(Some(name)) => Ok(name.to_string()),
      Ok(None) => Ok(format!(
        "Invalid utf string for branch name: {}",
        branch_name
      )),
      Err(e) => Ok(format!("Error : {}", e)),
    }
  }
}
