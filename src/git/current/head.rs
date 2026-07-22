use crate::git::Git;
use git2::Oid;
use git2::{ErrorCode, Repository};

#[allow(dead_code)]
pub enum Head {
  Refrence(String),
  Detached(Oid),
  Error(String),
  Unborn,
}

#[allow(dead_code)]
impl Head {
  pub fn is_refrence(&self) -> bool {
    matches!(self, Head::Refrence(_))
  }
  pub fn is_detached(&self) -> bool {
    matches!(self, Head::Detached(_))
  }
  pub fn is_error(&self) -> bool {
    matches!(self, Head::Error(_))
  }
  pub fn is_unborn(&self) -> bool {
    matches!(self, Head::Unborn)
  }
}

#[allow(dead_code)]
impl Git {
  /// Retuns enum `Head`.
  pub fn get_current_head(repo: &Repository) -> anyhow::Result<Head, anyhow::Error> {
    match repo.head() {
      Ok(head) => {
        if head.is_branch() {
          Ok(Head::Refrence(head.shorthand()?.to_string()))
        } else {
          match head.target() {
            Some(oid) => Ok(Head::Detached(repo.find_commit(oid)?.id())),
            None => Ok(Head::Error(
              "Detached HEAD but points to no Commit.".to_string(),
            )),
          }
        }
      }
      Err(e) if e.code() == ErrorCode::UnbornBranch => Ok(Head::Unborn),

      Err(e) => Ok(Head::Error(e.to_string())),
    }
  }

  // pub fn ahead_behind(repo: &Repository, head: &Head, local: &Local) -> anyhow::Result<()> {
  //   match &head {
  //     Head::Refrence(head) => {
  //       // let local_oid = head.target().unwrap();
  //       // let upstream_oid = Git::get_current_upstream(&local);
  //     }
  //     _ => {}
  //   }
  //   Ok(())
  // }
}
