use git2::{Branch, Commit, Reference, Repository};

pub mod current;
pub mod string_to_path;

/// Precidence : 0
/// Core struct -> holds expensive vars
/// It can't be life time bounded
#[allow(dead_code)]
pub struct Git {
  pub repo: Repository,
}

/// Precidence : 1st
///   
/// Refrence() -> current branch's -> head ref
/// Detached() -> no branch -> commit 
/// Error() -> Debug Error
/// Unborn -> Head is Unborn
#[allow(dead_code)]
pub enum Head<'repo> {
  Refrence(Reference<'repo>),
  Detached(Commit<'repo>),
  Error(String),
  Unborn,
}

/// Precidence: 2nd
/// If Head::Refrence(reference) -> `Branch<'repo>`
/// else -> `Error(String)`
#[allow(dead_code)]
pub enum Local<'repo> {
  Branch(Branch<'repo>),
  Error(String),
}

/// Precidence : 3rd
/// If `Local::Branch(Branch<'repo>)` -> `Commit<'repo>`.
/// else -> Error
#[allow(dead_code)]
pub enum Upstream<'repo> {
  Commit(Commit<'repo>),
  Error(String),
}

/// Precidence : 4th
/// Stores enum of Precidence 1, 2 and 3.
pub struct Current<'repo> {
  pub head_state: Head<'repo>,
  pub local_branch: Local<'repo>,
  pub upstream: Upstream<'repo>,
}

#[allow(dead_code)]
impl Git {
  pub fn new(path: &str) -> anyhow::Result<Self> {
    Ok(Self {
      repo: Repository::open(Git::string_to_path(path)?)?,
    })
  }
}
