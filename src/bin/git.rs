use pugit::git::{Git, current::local::Local};

fn main() -> anyhow::Result<()> {
  let git = Git::new("~/impl/rust/pugit/")?;

  if git.head.is_refrence() {
    match Git::get_current_local_branch(&git.head, &git.repo)? {
      Local::Branch(name) => println!("{}", name),
      Local::Error(error) => println!("{}", error),
      Local::None => println!("None"),
    }
  }
  Ok(())
}
