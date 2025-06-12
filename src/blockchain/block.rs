// Defines the Block structure for the blockchain.
// CORRECTED: The calculate_hash function is now public.

use crate::blockchain::transaction::Transaction;
use crate::utils::calculate_sha256;
use chrono::{DateTime, Utc};
use log::info;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub id: u64,
    pub timestamp: DateTime<Utc>,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
}

impl Block {
    /// Creates and mines a new block.
    pub fn new(id: u64, transactions: Vec<Transaction>, previous_hash: String, difficulty: &str) -> Self {
        let timestamp = Utc::now();
        let mut block = Block {
            id,
            timestamp,
            transactions,
            previous_hash,
            hash: String::new(),
            nonce: 0,
        };
        block.mine(difficulty);
        info!("New Block created and mined (ID: {}).", block.id);
        block
    }

    /// Calculates the SHA-256 hash of the block.
    /// This is now public so the Ledger can use it for validation.
    pub fn calculate_hash(&self) -> String {
        let mut data = self.id.to_string();
        data.push_str(&self.timestamp.to_rfc3339());
        data.push_str(&self.previous_hash);
        data.push_str(&self.nonce.to_string());
        for tx in &self.transactions {
            data.push_str(&tx.hash);
        }
        calculate_sha256(&data)
    }

    /// Simple Proof-of-Work mining function.
    fn mine(&mut self, difficulty: &str) {
        loop {
            let hash = self.calculate_hash();
            if hash.starts_with(difficulty) {
                self.hash = hash;
                return;
            }
            self.nonce += 1;
        }
    }
} impl Block {
    /// Validates the block's hash against its contents.
    pub fn validate(&self) -> bool {
        self.hash == self.calculate_hash()
    }
}