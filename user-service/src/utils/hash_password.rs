use argon2::{self, Argon2, PasswordHasher};
use argon2::password_hash::SaltString;

pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
  let salt = SaltString::generate(&mut rand::thread_rng());
  let argon2 = Argon2::default();

  let password_hash = argon2.hash_password(password.as_bytes(), &salt);

  Ok(password_hash.unwrap().to_string())
}
