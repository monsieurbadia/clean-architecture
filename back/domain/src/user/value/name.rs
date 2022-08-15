/// represents an name instance of the user's value
#[derive(Clone, Debug, PartialEq)]
pub struct Name(pub String);

impl From<&str> for Name {
  fn from(name: &str) -> Self {
    Self(name.into())
  }
}
