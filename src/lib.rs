//! src/lib.rs
//! Library root for Axiom Islam. Declares and exposes all system modules.

pub mod config;
pub mod data_models;
pub mod utils;

pub mod core {
    pub mod identity;
    pub mod neuron;
}

pub mod blockchain {
    pub mod block;
    pub mod ledger;
    pub mod transaction;
}

pub mod architecture_components {
    pub mod dsn;
    pub mod ksl;
    pub mod snn_quantum_pocket;
    pub mod topical_bnn;
}
