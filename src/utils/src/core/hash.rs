extern crate bcrypt;
use bcrypt::{DEFAULT_COST, hash, verify};

pub fn hash_password(password: &str) -> Result<String, String> {
    let hash = hash(password, DEFAULT_COST)
        .map_err(|err| err.to_string())?;
    Ok(hash)
}
pub fn verify_password(password: String, hash: &str) -> Result<bool, String> {
    let is_verified = verify(password, hash).unwrap();
    Ok(is_verified)
}