use crate::PathBuf;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chacha20poly1305::Key;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Write;

pub fn password_to_key() {}

pub fn create_db(name: &String, password: &String, file_path: &PathBuf) -> Key {
    //let full_path = format!("{}/{}.enc", file_path_string, name);
    //let path = file_path.as_ref().expect("no file path provided");

    let salt = SaltString::generate(&mut OsRng);
    let salt_bytes = salt.as_str().as_bytes();
    let mut file = File::create(file_path).expect("error");

    let mut key_bytes = [0u8; 32];

    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .expect("hash making error")
        .to_string();

    let _ = file.write_all(password_hash.as_bytes());

    argon2
        .hash_password_into(password.as_bytes(), &salt_bytes, &mut key_bytes)
        .expect("key make fail");

    return *Key::from_slice(&key_bytes);
}

pub fn unlock_db(
    password: &String,
    file_path: &Option<PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> {
    let path = match file_path.as_ref() {
        Some(p) => p,
        None => return Err("No file path selected".into()),
    };

    let mut file = File::open(path).map_err(|e| e.to_string())?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|e| e.to_string())?;

    let parsed_hash = match PasswordHash::new(&contents) {
        Ok(h) => h,
        Err(_) => return Err("Invalid password hash format".into()),
    };

    let argon2 = Argon2::default();

    match argon2.verify_password(password.as_bytes(), &parsed_hash) {
        Ok(_) => println!("Password verified successfully!"),
        Err(_) => return Err("Invalid password".into()),
    }

    Ok(())
}
