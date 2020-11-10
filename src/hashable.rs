use crypto_hash::{digest, Algorithm};

pub trait Hashable {
    fn bytes(&self) -> Vec<u8>;

    // creating a hash using SHA256 for the bytes from the object
    fn hash(&self) -> Vec<u8> {
        digest(Algorithm::SHA256, &self.bytes())
    }
}

