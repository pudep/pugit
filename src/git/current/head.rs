use git2::Oid;

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
