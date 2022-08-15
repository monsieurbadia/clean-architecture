use crate::user::value::email::Email;
use crate::user::value::name::Name;

/// represents a request of the user's creation
pub struct CreateUserRequest {
  pub name: Name,
  pub email: Email,
  pub is_member: bool,
}
