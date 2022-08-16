use anyhow::Error;

use crate::common::generator::uuid::UUIDGenerator;
use crate::user::entity::{User, UserId};
use crate::user::gateway::repository::UserRepository;

use super::request::CreateUserRequest;
use super::response::CreateUserResponse;

/// represents an instance of the create user use case
pub struct CreateUserUseCase {
  repository: Box<dyn UserRepository>,
}

impl CreateUserUseCase {
  /// creates a new instance of the create user use case
  pub fn new(repository: Box<dyn UserRepository>) -> Self {
    Self { repository }
  }

  /// executes a persistence of the user in the database
  pub fn execute(
    &mut self,
    request: CreateUserRequest,
  ) -> Result<CreateUserResponse, Error> {
    let uuid = UUIDGenerator::generate();
    let id = UserId::from(uuid.as_str());
    let user = User::new(id, request.name, request.email, request.is_member);

    self.repository.create(user)
  }
}

#[cfg(test)]
mod tests {
  use crate::common::generator::uuid::UUIDGenerator;
  use crate::user::entity::{User, UserId};
  use crate::user::gateway::repository::MockUserRepository;
  use crate::user::usecase::create::request::CreateUserRequest;
  use crate::user::value::email::Email;
  use crate::user::value::name::Name;

  use super::CreateUserUseCase;

  #[test]
  fn should_create_a_user() {
    let user_request = CreateUserRequest {
      name: Name::from("foobar"),
      email: Email::from("foobar@user.com"),
      is_member: false,
    };

    let mut user_mock = MockUserRepository::new();

    user_mock.expect_create().times(1).returning(|_| {
      let uuid = UUIDGenerator::generate();

      Ok(User::new(
        UserId::from(uuid.as_str()),
        Name::from("foobar"),
        Email::from("foobar@user.com"),
        false,
      ))
    });

    let mut user_usecase = CreateUserUseCase::new(Box::new(user_mock));
    let user = user_usecase.execute(user_request).unwrap();

    assert_eq!(user.email, Email::from("foobar@user.com"))
  }

  // TODO should not create user if already exists
}
