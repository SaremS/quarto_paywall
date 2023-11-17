use scrypt::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Scrypt,
};

pub fn get_hash(hashable: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);

    let hash = Scrypt
        .hash_password(hashable.as_bytes(), &salt)
        .unwrap()
        .to_string();

    return hash;
}

pub fn get_hash_fixed_salt(hashable: &str) -> String {
    let salt = SaltString::from_b64(&"ASDF").unwrap();

    let hash = Scrypt
        .hash_password(hashable.as_bytes(), &salt)
        .unwrap()
        .to_string();

    return hash;
}

pub fn verify_hash(hashable: &str, hash: &str) -> bool {
    let parsed_hash = PasswordHash::new(hash).unwrap();

    return Scrypt
        .verify_password(hashable.as_bytes(), &parsed_hash)
        .is_ok();
}

pub fn xor_hash(s: &str) -> String {
    /*
    Trivial hashing to create unique identifiers that don't need to be secure.
    */
    let mut hash = 0u8;
    for byte in s.bytes() {
        hash ^= byte;
    }
    return format!("{:02x}", hash);
}

pub fn xor_cipher(input: &str, key: u8) -> String {
    return input.chars().map(|c| (c as u8 ^ key) as char).collect();
}