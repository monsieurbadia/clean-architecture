use anyhow::Error;
use mockall::*;

use crate::user::entity::User;

/// represents an interface of the user repository
#[automock]
pub trait UserRepository {
  /// creates a new user in the database
  fn create(&mut self, user: User) -> Result<User, Error>;
  // TODO: find, update, delete
}
