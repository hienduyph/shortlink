use argon2::password_hash::rand_core::OsRng;
use argon2::PasswordHasher;
use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordVerifier};

pub fn verify(raw: &str, hashed: String) -> bool {
    let ag = Argon2::default();
    match PasswordHash::new(&hashed) {
        Ok(parsed) => ag.verify_password(raw.as_bytes(), &parsed).is_ok(),
        Err(err) => {
            println!("{}", err);
            false
        }
    }
}

pub type Error = argon2::password_hash::Error;

pub fn generate(pwd: &str) -> Result<(String, String), Error> {
    let salt = SaltString::generate(&mut OsRng);
    let ag = Argon2::default();
    let pwd_hashed = ag.hash_password(pwd.as_bytes(), &salt)?.to_string();
    return Ok((pwd_hashed, salt.as_str().to_string()));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn should_ok() {
        let pwd = "hello this is my password";
        let (hashed, _salt) = generate(pwd).unwrap();
        let ok = verify(pwd, hashed.clone());
        assert!(ok, "Verify sould ok");

        let another = "not my password";
        assert!(!verify(another, hashed), "this should not match");
    }
}
