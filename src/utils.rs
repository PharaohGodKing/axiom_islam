//! src/utils.rs
//! Provides common utility functions.

use sha2::{Sha256, Digest};

/// Generates a SHA-256 hash for a given string.
pub fn calculate_sha256(data: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}