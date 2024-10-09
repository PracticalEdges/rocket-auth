use rand::{Rng, thread_rng};
use rand:: distributions::Alphanumeric;
use sha2::{Sha256, Digest};

pub fn generate_random_hash_function(len: usize) -> String {
    let random_string: String = thread_rng()
    .sample_iter(&Alphanumeric)
    .take(len)
    .map(char::from)
    .collect();

    let mut hasher = Sha256::new();

    hasher.update(random_string);

    let result = hasher.finalize();

    format!("{:x}", result)[..len].to_string()
}