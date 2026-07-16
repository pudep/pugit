use crate::git::Head;
use crate::git::Local;
use crate::git::Upstream;

use crate::git::Git;
use git2::{ ErrorCode, Repository};
#[allow(dead_code)]
impl Git {
  /// Retuns enum `Head`.
  pub fn get_current_head<'repo>(
    repo: &'repo Repository,
  ) -> anyhow::Result<Head<'repo>, anyhow::Error> {
    match repo.head() {
      // A head (latest commit) can point either to a Branch say Main or to a commit(only if is detached) so :
      Ok(head) => {
        if head.is_branch() {
          Ok(Head::Refrence(head))
        } else {
          match head.target() {
            Some(oid) => Ok(Head::Detached(repo.find_commit(oid)?)),
            None => Ok(Head::Error(
              "Detached HEAD but points to no Commit.".to_string(),
            )),
          }
        }
      }
      // This is done to tell user that the Branch is unborn.
      Err(e) if e.code() == ErrorCode::UnbornBranch => Ok(Head::Unborn),

      // This displays serious to resolve errors.
      Err(e) => Ok(Head::Error(e.to_string())),
    }
  }

  /// Validity of `Branch<'_>` depends on `Repository`.
  /// Vulnerable to staling.
  pub fn get_current_local_branch<'repo>(
    repo: &'repo Repository,
    head_state: &Head,
  ) -> Local<'repo> {
    match head_state {
      Head::Refrence(refrence) => {
        match repo.find_branch(refrence.shorthand().unwrap(), git2::BranchType::Local) {
          Ok(b) => Local::Branch(b),
          Err(e) => Local::Error(e.to_string()),
        }
      }
      _ => Local::Error("The Head::Refrence(refrence) was likely invalid".to_string()),
    }
  }

  /// Get latest oid of the local branch which has been pushed to the underlying remote.
  pub fn get_oid_current<'repo>(
    repo: &'repo Repository,
    current: &Local,
  ) -> anyhow::Result<Upstream<'repo>, anyhow::Error> {
    match current {
      Local::Branch(b) => match b.upstream() {
        Ok(localbranch) => Ok(Upstream::Commit(
          repo.find_commit(localbranch.get().target().unwrap())?,
        )),
        Err(e) => Ok(Upstream::Error(e.to_string())),
      },
      Local::Error(error) => Ok(Upstream::Error(error.to_string())),
    }
  }

  /// This will return the `refs/remote/upstream` for current local branch.
  /// E.g : Local branch -> `main`
  ///       Upstream -> `origin/main`
  pub fn get_current_upstream(current: &Local) -> anyhow::Result<String> {
    match &current {
      Local::Branch(local_branch) => Ok(
        local_branch
          .upstream()?
          .name()?
          .unwrap_or("<No Remote>")
          .to_string(),
      ),
      _ => Ok("Nil".to_string()),
    }
  }

  pub fn unk(){
  }
}
