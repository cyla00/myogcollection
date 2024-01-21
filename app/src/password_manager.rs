use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, SaltString, PasswordVerifier,
    },
    Argon2
};

pub fn password_hashing(password: String) -> (String, SaltString) {
    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default().hash_password(&password.as_bytes(), &salt).unwrap().to_string();
    (hashed_password.to_string(), salt.clone())
}

pub fn password_verification(old_password: String, new_password: String) -> bool {
    let old_pass_hash = PasswordHash::new(&old_password).unwrap();
    let pass_verification = Argon2::default().verify_password(&new_password.as_bytes(), &old_pass_hash);

    match pass_verification {
        Ok(_) => return true,
        Err(_) => return false,
    }
}