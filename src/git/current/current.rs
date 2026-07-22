use crate::git::Head;

use crate::git::Git;
use crate::git::current::local::Local;
use git2::{ErrorCode, Repository};
#[allow(dead_code)]
impl Git {
  /// Retuns enum `Head`.
  pub fn get_current_head(repo: &Repository) -> anyhow::Result<Head, anyhow::Error> {
    match repo.head() {
      // A head (latest commit) can point either to a Branch say Main or to a commit(only if is detached) so :
      Ok(head) => {
        if head.is_branch() {
          Ok(Head::Refrence(head.name()?.to_string()))
        } else {
          match head.target() {
            Some(oid) => Ok(Head::Detached(repo.find_commit(oid)?.id())),
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

  /// Get latest oid of the local branch which has been pushed to the underlying remote.
  pub fn ahead_behind(repo: &Repository, head: &Head, local: &Local) -> anyhow::Result<()> {
    match &head {
      Head::Refrence(head) => {
        // let local_oid = head.target().unwrap();
        let upstream_oid = Git::get_current_upstream(&local);
      }
      _ => {}
    }
    Ok(())
  }
}
