use crate::user::value::email::Email;
use crate::user::value::name::Name;

/// represents an instance of the user
#[derive(Clone, Debug, PartialEq)]
pub struct User {
  pub id: UserId,
  pub name: Name,
  pub email: Email,
  pub is_member: bool,
}

impl User {
  /// creates a new instance of the user
  pub fn new(id: UserId, name: Name, email: Email, is_member: bool) -> Self {
    Self {
      id,
      name,
      email,
      is_member,
    }
  }

  // TODO: check the validity of the name and email
}

/// represents an instance of the user id
#[derive(Clone, Debug, PartialEq)]
pub struct UserId(pub String);

impl From<&str> for UserId {
  fn from(id: &str) -> Self {
    Self(id.into())
  }
}

#[cfg(test)]
mod tests {
  use crate::common::generator::uuid::UUIDGenerator;
  use crate::user::entity::user::UserId;
  use crate::user::value::email::Email;
  use crate::user::value::name::Name;

  use super::User;

  #[test]
  fn user_should_build() {
    let uuid = UUIDGenerator::generate();
    let id = uuid.as_str();

    let user = User::new(
      UserId::from(id),
      Name::from("foobar"),
      Email::from("foobar@user.com"),
      false,
    );

    assert_eq!(user.id, UserId::from(id));
    assert_eq!(user.name, Name::from("foobar"));
    assert_eq!(user.email, Email::from("foobar@user.com"));
    assert_eq!(user.is_member, false);
  }
}
