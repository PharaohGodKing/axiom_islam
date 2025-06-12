//! src/config.rs
//! Defines the SystemConfig struct for global configuration parameters.

use serde::{Deserialize, Serialize};
// CORRECTED: Removed unused `std::fs` and `std::io::{self, Read}` imports.
use bincode::{Encode, Decode}; 

// Removed `Default` from derive macro as it's manually implemented.
#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct SystemConfig {
    // --- Data Paths ---
    pub data_root_path: String,
    pub core_philosophies_data_path: String,
    pub key_projects_data_path: String,
    pub user_profile_data_path: String,
    pub cosmology_data_path: String,
    pub bio_hybrid_data_path: String,

    // --- Architectural Counts ---
    pub ksl_count: usize,
    pub dsns_per_ksl: usize,
    pub bnns_per_dsn: usize,
    pub qps_per_bnn: usize, 

    // --- Neuron/SNN Parameters ---
    pub snn_quantum_pocket_default_neurons: usize,
    pub bnn_default_neurons: usize, 
    pub default_learning_rate: f64,
    pub snn_processing_cycles: u32,
    pub lif_neuron_leak_rate: f64,
    pub lif_neuron_threshold: f64,
    pub lif_neuron_reset_potential: f64,
    pub lif_neuron_refractory_cycles: u32,
    pub lif_neuron_base_power_mw: f64,
    pub neuron_activity_multiplier: f64,

    // --- Thresholds & Ratios ---
    pub dsn_pol_threshold: f64,
    pub bnn_consensus_threshold: f64, 

    // --- Axiom Resonance Algorithm (ARA) Parameters ---
    pub ara_ethical_violation_penalty_coherence: f64,
    pub ara_matria_operation_strength: f64,
    pub ara_base_sadness_threshold: f64,
    pub ara_base_joy_threshold: f64,

    // --- Network / System Parameters ---
    pub network_broadcast_delay_ms: u32,
    pub architect_key: String,
}

impl SystemConfig {
    /// Loads the system configuration from a JSON file.
    pub fn load_from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file = std::fs::File::open(path)?;
        let reader = std::io::BufReader::new(file);
        Ok(serde_json::from_reader(reader)?)
    }

    /// Saves the current configuration to a file in JSON format.
    pub fn save_to_file(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let file = std::fs::File::create(path)?;
        let writer = std::io::BufWriter::new(file);
        Ok(serde_json::to_writer(writer, self)?)
    }

    /// Creates a new `SystemConfig` with default values matching your provided JSON.
    pub fn new() -> Self {
        SystemConfig {
            data_root_path: String::from("data/"),
            core_philosophies_data_path: String::from("core_philosophies.json"),
            key_projects_data_path: String::from("key_projects.json"),
            user_profile_data_path: String::from("user_profile.json"),
            cosmology_data_path: String::from("cosmology_data.json"),
            bio_hybrid_data_path: String::from("bio_hybrid_data.json"),
            
            // --- UPDATED NUMERIC VALUES TO MATCH YOUR PROVIDED JSON ---
            ksl_count: 12,
            dsns_per_ksl: 36,
            bnns_per_dsn: 18,
            qps_per_bnn: 21,
            snn_quantum_pocket_default_neurons: 21,
            bnn_default_neurons: 21,
            default_learning_rate: 0.001,
            dsn_pol_threshold: 0.75,
            bnn_consensus_threshold: 0.65,
            snn_processing_cycles: 333,
            lif_neuron_leak_rate: 75.0,
            lif_neuron_threshold: 5.0,
            lif_neuron_reset_potential: 0.0,
            lif_neuron_refractory_cycles: 3,
            lif_neuron_base_power_mw: 1e-11,
            ara_ethical_violation_penalty_coherence: 0.15,
            ara_matria_operation_strength: 0.08,
            ara_base_sadness_threshold: 0.5,
            ara_base_joy_threshold: 0.5,
            network_broadcast_delay_ms: 10,
            neuron_activity_multiplier: 5.0,
            architect_key: String::from("0K12291033b19l21e0UU@#$")
            // --- END UPDATED NUMERIC VALUES ---
        }
    }
}

// Implementing the `Default` trait for `SystemConfig` to provide default values.
impl Default for SystemConfig {
    fn default() -> Self {
        SystemConfig::new()
    }
}