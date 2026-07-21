use crate::git::Head;
use crate::git::Local;
use crate::git::RepoState;
use crate::git::Upstream;

use crate::git::Git;
use git2::RepositoryState;
use git2::{ErrorCode, Repository};
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

  pub fn get_repo_state(repo: &Repository) -> RepoState {
    let state = repo.state();
    match &state {
      RepositoryState::Clean => RepoState::Clean,
      RepositoryState::Merge => RepoState::Merging,
      RepositoryState::Revert => RepoState::SingleRevert,
      RepositoryState::RevertSequence => RepoState::MultiRevert,
      RepositoryState::CherryPick => RepoState::SingleCherryPick,
      RepositoryState::CherryPickSequence => RepoState::MultiCherryPick,
      RepositoryState::RebaseMerge => RepoState::RebaseMerge,
      RepositoryState::Rebase | RepositoryState::RebaseInteractive => RepoState::Rebasing,
      RepositoryState::Bisect => RepoState::Bisect,
      RepositoryState::ApplyMailbox => RepoState::ApplyingMailBox,
      RepositoryState::ApplyMailboxOrRebase => RepoState::MailBoxOrRebase,
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
        // We give priority to Local Branch when it comes to look at current branch.
        // If current branch contains a remote ref then we can later using the name find the remote branch.
        // Moroever all main code lives on Local hence it worths checking more than Remote.
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
  /// branch_name.upstream()? tells which Remote branch do Local one is tracking?
  pub fn get_current_upstream<'repo>(repo: &'repo Repository, current: &Local) -> Upstream<'repo> {
    match &current {
      Local::Branch(local_branch) => match local_branch.upstream() {
        Ok(k) => Upstream::Branch(k),
        Err(e) => Upstream::Error(e.to_string()),
      },
      _ => Upstream::None,
    }
  }

  pub fn ahead_behind(repo: &Repository, head: &Head, local: &Local) -> anyhow::Result<()> {
    match &head {
      Head::Refrence(head) => {
        let local_oid = head.target().unwrap();
        let upstream_oid = Git::get_current_upstream(local)?;
      }
      _ => {}
    }
    Ok(())
  }
}
