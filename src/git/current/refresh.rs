// use git2::Repository;
//
// use crate::{git::Git, watcher::WatchSignals};
// fn refresh(repo: &Repository, signal: WatchSignals) -> anyhow::Result<()> {
//   if signal
//     .head_changed
//     .swap(false, std::sync::atomic::Ordering::Relaxed)
//   {
//     let head = Git::get_current_head(repo)?;
//     let branch = Git::get_current_local_branch(repo, &head);
//     let oid = Git::get_oid_current(repo, &branch)?;
//     let upstream = Git::get_current_upstream(&branch);
//   }
//
//   Ok(())
// }
