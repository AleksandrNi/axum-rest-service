pub fn hash_password(password: &str) -> Result<String, String> {
    let hash = bcrypt::hash(password, 14)
        .map_err(|err| err.to_string())?;
    Ok(hash)
}
pub fn verify_password(password: String, hash: &str) -> Result<bool, String> {
    let is_verified = bcrypt::verify(password, hash).unwrap();
    Ok(is_verified)
}