use sha2::{Sha256, Digest};

pub fn sha256(message: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(message);
    let result = hasher.finalize();
    let string = format!("{:x}", result);
    String::from(string)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_sha256() {
        assert_eq!(sha256("karan"), String::from("46ed260db5a4cb33871f0b308aae3e899602cd7f20c6841677e4079d8b9e5ec3"));
    }

}
