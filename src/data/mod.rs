use std::error::Error;

#[allow(dead_code)]
pub struct GitHealth{

}

#[allow(dead_code)]
impl GitHealth {
  fn git() -> Result<(), Box<dyn Error>>{
    let repo = git2::Repository::open("~/impl/rust/pubat/")?;
    let head: git2::Reference = repo.head()?;
    let head_commit = head.peel_to_commit()?;
    let branch_name = head.shorthand();
    let head_is_detached = repo.head_detached();

    let mut status = git2::StatusOptions::new();
    status.include_untracked(true).recurse_untracked_dirs(true).include_ignored(true);

    let statuses = repo.statuses(Some(&mut status))?;

    for entry in statuses.iter() {
      let path = entry.path().unwrap_or("");
      let status = entry.status();

      // let is_newstatus.is_index_new();
    }
    Ok(())
  }
}
