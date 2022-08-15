use uuid::Uuid;

/// represents an instance of the UUID generator
pub struct UUIDGenerator;

impl UUIDGenerator {
  /// creates a new UUID as a string
  pub fn generate() -> String {
    Uuid::new_v4().to_string()
  }
}
