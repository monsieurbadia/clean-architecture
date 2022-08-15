use anyhow::Error;

use crate::user::entity::User;

/// represents an interface of the user repository
pub trait UserRepository {
  /// creates a new user in the database
  fn create(&mut self, user: User) -> Result<User, Error>;
  // TODO: find, update, delete
}
