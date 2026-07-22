use git2::{Branch, Commit, Oid, Repository};

pub mod current;
pub mod refresh;
pub mod string_to_path;

#[allow(dead_code)]
pub struct Git {
  pub repo: Repository,
  pub is: Is, 
}

// Helps doing if else based checks when ever we dont want to match Enum's result.
pub struct Is {
  pub head: bool,
}

pub enum RepoState {
  Clean,
  Merging,
  Rebasing,
  RebaseMerge,
  SingleCherryPick,
  MultiCherryPick,
  SingleRevert,
  MultiRevert,
  Bisect,
  ApplyingMailBox,
  MailBoxOrRebase,
}

#[allow(dead_code)]
pub enum Head {
  Refrence(String),
  Detached(Oid),
  Error(String),
  Unborn,
}

#[allow(dead_code)]
pub enum Local<'repo> {
  Branch(Branch<'repo>),
  Error(String),
}

#[allow(dead_code)]
pub enum Upstream<'repo> {
  Branch(Branch<'repo>),
  Commit(Commit<'repo>),
  Error(String),
  None,
}

#[allow(dead_code)]
impl Git {
  pub fn new(path: &str) -> anyhow::Result<Self> {
    Ok(Self {
      repo: Repository::open(Git::string_to_path(path)?)?,
      is: Is { head: false },
    })
  }
}
