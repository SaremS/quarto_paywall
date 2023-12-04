use scrypt::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Scrypt,
};

pub trait HashingAlgorithm: Sync + Send {
    fn get_hash(hashable: &str) -> String;
    fn verify_hash(hashable: &str, hash: &str) -> bool;
}

///uses Scrypt hashing algorithm
pub struct ScryptHashing {}

impl HashingAlgorithm for ScryptHashing {
    fn get_hash(hashable: &str) -> String {
        let salt = SaltString::generate(&mut OsRng);

        let hash = Scrypt
            .hash_password(hashable.as_bytes(), &salt)
            .unwrap()
            .to_string();

        return hash;
    }

    fn verify_hash(hashable: &str, hash: &str) -> bool {
        let parsed_hash = PasswordHash::new(hash).unwrap();

        return Scrypt
            .verify_password(hashable.as_bytes(), &parsed_hash)
            .is_ok();
    }
}
 
///Returns input as hash ('identity hash function') - useful for testing since
///scrypt is quite slow outside of release builds
pub struct NonHashing{}

impl HashingAlgorithm for NonHashing {
    fn get_hash(hash: &str) -> String {
        return hash.to_string();
    }

    fn verify_hash(hashable: &str, hash: &str) -> bool {
        return hashable == hash;
    }
}

fn get_hash_fixed_salt(hashable: &str) -> String {
    let salt = SaltString::from_b64(&"ASDF").unwrap();

    let hash = Scrypt
        .hash_password(hashable.as_bytes(), &salt)
        .unwrap()
        .to_string();

    return hash;
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
