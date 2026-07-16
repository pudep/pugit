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
/// If current branch -> ref head
/// else : `Commit<'repo>`
/// If Unborn -> `Unborn`
/// Rare : Error
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

#[allow(dead_code)]
impl Git {
  pub fn new(path: &str) -> anyhow::Result<Self> {
    Ok(Self {
      repo: Repository::open(Git::string_to_path(path)?)?,
    })
  }
}
