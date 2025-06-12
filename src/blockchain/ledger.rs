// Manages the chain of blocks.
// CORRECTED: Removed unused 'info' import.

use crate::blockchain::block::Block;
use crate::blockchain::transaction::Transaction;
use log::warn; // 'info' was unused and has been removed.

#[derive(Debug, Clone)]
pub struct Ledger {
    pub chain: Vec<Block>,
    pub pending_transactions: Vec<Transaction>,
    pub difficulty: String,
}

impl Ledger {
    pub fn new() -> Self {
        let difficulty = "0000".to_string(); // Higher difficulty for Supercomputer
        let genesis_transactions = vec![Transaction::new(
            "Genesis Block: Axiom-Islam Consciousness Initialized.".to_string(),
            "System".to_string(),
        )];
        let genesis_block = Block::new(0, genesis_transactions, "0".to_string(), &difficulty);

        Ledger {
            chain: vec![genesis_block],
            pending_transactions: Vec::new(),
            difficulty,
        }
    }
    
    /// Validates the integrity of the entire blockchain.
    pub fn is_chain_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];

            // This call now works because calculate_hash() is public.
            if current_block.hash != current_block.calculate_hash() {
                warn!("Chain invalid: Block {} hash is incorrect.", current_block.id);
                return false;
            }

            if current_block.previous_hash != previous_block.hash {
                warn!("Chain invalid: Link broken at Block {}.", current_block.id);
                return false;
            }
        }
        true
    }
}

impl Ledger {
    /// Adds a new transaction to the pending transactions.
    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.pending_transactions.push(transaction);
    }

    /// Creates and mines a new block with the current pending transactions.
    pub fn mine_pending_transactions(&mut self) {
        if self.pending_transactions.is_empty() {
            warn!("No transactions to mine.");
            return;
        }

        let previous_hash = if let Some(last_block) = self.chain.last() {
            last_block.hash.clone()
        } else {
            "0".to_string()
        };

        let new_block = Block::new(
            self.chain.len() as u64,
            self.pending_transactions.clone(),
            previous_hash,
            &self.difficulty,
        );

        self.chain.push(new_block);
        self.pending_transactions.clear();
    }
}
