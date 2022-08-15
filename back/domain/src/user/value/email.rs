/// represents an email instance of the user's value
#[derive(Clone, Debug, PartialEq)]
pub struct Email(pub String);

impl From<&str> for Email {
  fn from(email: &str) -> Self {
    Self(email.into())
  }
}
