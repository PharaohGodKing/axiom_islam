//! src/blockchain/transaction.rs
//! Defines the Transaction structure for blockchain operations.

use crate::utils::calculate_sha256;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

/// Represents a transaction in the blockchain.
/// 
/// Each transaction contains:
/// - A unique identifier
/// - A timestamp of when it was created
/// - Content data
/// - Information about the sender
/// - A cryptographic hash for verification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub content: String,
    pub sender: String,
    pub hash: String,
}

impl Transaction {
    /// Creates a new transaction with the given content and sender.
    ///
    /// # Arguments
    ///
    /// * `content` - The content of the transaction
    /// * `sender` - The identifier of the transaction sender
    ///
    /// # Returns
    ///
    /// A new Transaction with generated id, timestamp, and hash
    pub fn new(content: String, sender: String) -> Self {
        let timestamp = Utc::now();
        let id = Uuid::new_v4().to_string();
        
        // Use string concatenation with capacity for better performance
        let data_len = id.len() + timestamp.to_rfc3339().len() + content.len() + sender.len();
        let mut data_to_hash = String::with_capacity(data_len);
        data_to_hash.push_str(&id);
        data_to_hash.push_str(&timestamp.to_rfc3339());
        data_to_hash.push_str(&content);
        data_to_hash.push_str(&sender);

        let hash = calculate_sha256(&data_to_hash);

        Transaction {
            id,
            timestamp,
            content,
            sender,
            hash,
        }
    }

    /// Verifies that the transaction's hash is valid.
    ///
    /// Recalculates the hash based on the transaction's data and
    /// compares it with the stored hash to ensure integrity.
    ///
    /// # Returns
    ///
    /// `true` if the hash is valid, `false` otherwise
    pub fn verify_hash(&self) -> bool {
        let mut data_to_hash = String::with_capacity(
            self.id.len() + self.timestamp.to_rfc3339().len() + self.content.len() + self.sender.len()
        );
        data_to_hash.push_str(&self.id);
        data_to_hash.push_str(&self.timestamp.to_rfc3339());
        data_to_hash.push_str(&self.content);
        data_to_hash.push_str(&self.sender);
        
        let calculated_hash = calculate_sha256(&data_to_hash);
        self.hash == calculated_hash
    }
    
    /// Returns the transaction's hash.
    pub fn hash(&self) -> &str {
        &self.hash
    }
    
    /// Returns the transaction's ID.
    pub fn id(&self) -> &str {
        &self.id
    }
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Transaction {{ id: {}, timestamp: {}, sender: {}, content_length: {} }}",
            self.id,
            self.timestamp,
            self.sender,
            self.content.len()
        )
    }
}
