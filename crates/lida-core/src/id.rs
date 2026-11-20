//! 4-character base62 collision-free ID generator.

use std::collections::HashSet;

const ALPHABET: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

/// Generates a random 4-character alphanumeric ID not present in `existing`.
#[must_use]
pub fn generate_id<S: std::hash::BuildHasher>(existing: &HashSet<String, S>) -> String {
    loop {
        let mut bytes = [0u8; 4];
        for b in &mut bytes {
            let idx = fastrand::usize(..ALPHABET.len());
            *b = ALPHABET[idx];
        }
        let candidate = String::from_utf8_lossy(&bytes).to_string();
        if !existing.contains(&candidate) {
            return candidate;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id_structure() {
        let set = HashSet::new();
        let id = generate_id(&set);
        assert_eq!(id.len(), 4);
    }
}
