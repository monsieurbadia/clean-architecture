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
