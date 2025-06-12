//! src/architecture_components/snn_quantum_pocket.rs
//! Defines the Spiking Neural Network (SNN) Quantum Pocket.

use bincode::{BorrowDecode, Decode, Encode}; // Added BorrowDecode
use crate::config::SystemConfig;
use crate::core::neuron::LIFNeuron;
use crate::data_models::{ExternalDataPoint, SerializableDateTimeUtc, SerializableUuid, SpikeEvent}; // Correct import
use chrono::Utc;
use log::info;
use rand::prelude::*; // Provides traits like Rng for `choose_multiple`
use rand::rngs::ThreadRng; // Explicit import for ThreadRng
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

/// Represents a snapshot of the SNN Quantum Pocket's state after processing data.
/// Indicates whether anomalies or patterns were detected.
pub struct QPSnapshot {
    pub anomaly_detected: bool,
    pub pattern_detected: bool,
}

/// Helper function to provide a default empty HashMap for neurons when deserializing.
fn default_neurons() -> Arc<Mutex<HashMap<SerializableUuid, LIFNeuron>>> {
    Arc::new(Mutex::new(HashMap::new()))
}

/// Represents a Spiking Neural Network (SNN) Quantum Pocket.
///
/// This component simulates a sub-network of neurons designed for specific
/// pattern recognition and anomaly detection tasks. It's part of the broader
/// cognitive architecture.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnnQuantumPocket {
    pub id: SerializableUuid,
    #[serde(skip, default = "default_neurons")] // Skip serialization/deserialization for the Arc<Mutex<HashMap>>
    pub neurons: Arc<Mutex<HashMap<SerializableUuid, LIFNeuron>>>,
    pub config: SystemConfig,
}

// Manually implement Encode/Decode because `neurons` field is skipped by serde.
// This ensures bincode only attempts to serialize `id` and `config`.
impl Encode for SnnQuantumPocket {
    fn encode<E: bincode::enc::Encoder>(&self, encoder: &mut E) -> Result<(), bincode::error::EncodeError> {
        self.id.encode(encoder)?;
        self.config.encode(encoder)?;
        Ok(())
    }
}

// Manually implement Decode. When decoding, `neurons` will be re-initialized
// using the configuration, simulating a fresh start for the SNN within the pocket.
impl<C> Decode<C> for SnnQuantumPocket {
    fn decode<D: bincode::de::Decoder<Context = C>>(decoder: &mut D) -> Result<Self, bincode::error::DecodeError> {
        let id: SerializableUuid = Decode::decode(decoder)?;
        let config: SystemConfig = Decode::decode(decoder)?;
        // When loading, create a new pocket which re-initializes its neurons based on config.
        let mut new_pocket = SnnQuantumPocket::new(config); 
        new_pocket.id = id; // Retain the original ID
        Ok(new_pocket)
    }
}

// Manually implement BorrowDecode, similar logic to Decode.
impl<'de, C: Sized> BorrowDecode<'de, C> for SnnQuantumPocket {
    fn borrow_decode<D: bincode::de::BorrowDecoder<'de, Context = C>>(decoder: &mut D) -> Result<Self, bincode::error::DecodeError> {
        let id: SerializableUuid = BorrowDecode::borrow_decode(decoder)?;
        let config: SystemConfig = BorrowDecode::borrow_decode(decoder)?;
        let mut new_pocket = SnnQuantumPocket::new(config);
        new_pocket.id = id;
        Ok(new_pocket)
    }
}


impl SnnQuantumPocket {
    /// Creates a new SnnQuantumPocket instance.
    ///
    /// Initializes a specified number of LIF neurons and forms basic connections
    /// between them based on the system configuration.
    ///
    /// # Arguments
    /// * `config` - The `SystemConfig` to use for neuron initialization and connection.
    ///
    /// # Returns
    /// A new `SnnQuantumPocket` instance.
    pub fn new(config: SystemConfig) -> Self {
        let mut neurons_map = HashMap::new();
        // Create initial neurons for the pocket.
        for _ in 0..config.snn_quantum_pocket_default_neurons {
            let neuron = LIFNeuron::new(&config);
            neurons_map.insert(neuron.id.clone(), neuron);
        }
        let neuron_ids: Vec<SerializableUuid> = neurons_map.keys().cloned().collect();

        // Form connections between neurons.
        for id in &neuron_ids {
            if let Some(neuron) = neurons_map.get_mut(id) {
                // Allows deprecated calls to rand::thread_rng if Cargo.toml has an older rand version.
                #[allow(deprecated)] 
                let mut rng: ThreadRng = rand::thread_rng(); 
                let connections_to_make = (neuron_ids.len() / 10).max(1); // Determine number of connections
                let potential_targets: Vec<&SerializableUuid> =
                    neuron_ids.iter().filter(|target_id| *target_id != id).collect();

                if !potential_targets.is_empty() {
                    let num_to_choose = connections_to_make.min(potential_targets.len());
                    // Choose random targets to form connections.
                    let targets: Vec<SerializableUuid> = potential_targets
                        .choose_multiple(&mut rng, num_to_choose)
                        .map(|&target| target.clone())
                        .collect();
                    for target_id in targets {
                        neuron.form_connection(target_id);
                    }
                }
            }
        }
        SnnQuantumPocket {
            id: SerializableUuid(Uuid::new_v4()),
            neurons: Arc::new(Mutex::new(neurons_map)),
            config,
        }
    }

    /// Re-initializes the configuration for the SNN Quantum Pocket and its contained neurons.
    ///
    /// This method ensures that configuration changes are propagated to the SNN.
    ///
    /// # Arguments
    /// * `config` - The new `SystemConfig` to apply.
    pub fn re_init_config(&mut self, config: SystemConfig) {
        self.config = config.clone();
        if let Ok(mut neurons_guard) = self.neurons.lock() {
            for neuron in neurons_guard.values_mut() {
                // Update individual neuron parameters from the new config
                neuron.threshold = config.lif_neuron_threshold;
                neuron.leak_rate = config.lif_neuron_leak_rate;
                neuron.reset_potential = config.lif_neuron_reset_potential;
                neuron.refractory_cycles = config.lif_neuron_refractory_cycles;
                neuron.learning_rate = config.default_learning_rate;
            }
        }
    }

    /// Processes an external data point through the SNN Quantum Pocket.
    ///
    /// This simulates the network reacting to input by potentially causing neurons to fire.
    /// It returns a `QPSnapshot` indicating detected anomalies or patterns.
    ///
    /// # Arguments
    /// * `data` - The `ExternalDataPoint` to process.
    ///
    /// # Returns
    /// A `QPSnapshot` summarizing the processing results (anomaly/pattern detection).
    pub async fn process_data(&mut self, data: ExternalDataPoint) -> QPSnapshot {
        let mut total_fires = 0;
        // Calculate input value based on data content length and a multiplier from config.
        let input_value = (data.content.len() as f64 * 0.1) * self.config.neuron_activity_multiplier;
        
        // Simulate processing cycles for the SNN.
        for cycle in 0..self.config.snn_processing_cycles {
            let mut firings_this_cycle: HashMap<SerializableUuid, SpikeEvent> = HashMap::new();
            if let Ok(mut neurons_guard) = self.neurons.lock() {
                for (id, neuron) in neurons_guard.iter_mut() {
                    if cycle == 0 {
                        // Apply initial input to neurons in the first cycle.
                        neuron.membrane_potential += input_value;
                    }
                    if neuron.update_and_check_fire() {
                        total_fires += 1;
                        let spike = SpikeEvent {
                            presynaptic_neuron_id: id.clone(),
                            timestamp: SerializableDateTimeUtc(Utc::now()),
                            value: 1.0, // A simple spike value
                        };
                        firings_this_cycle.insert(id.clone(), spike);
                    }
                }

                // Propagate spikes and apply STDP for learning within the current cycle.
                if !firings_this_cycle.is_empty() {
                    let spikes: Vec<SpikeEvent> = firings_this_cycle.values().cloned().collect();
                    let firing_keys: Vec<SerializableUuid> =
                        firings_this_cycle.keys().cloned().collect();
                    for firing_neuron_id in firing_keys {
                        if let Some(firing_neuron) = neurons_guard.get(&firing_neuron_id) {
                            let firing_neuron_connections = firing_neuron.connections.clone();
                            if let Some(event) = firings_this_cycle.get(&firing_neuron_id) {
                                for (target_neuron_id, weight) in firing_neuron_connections {
                                    if let Some(target_neuron) =
                                        neurons_guard.get_mut(&target_neuron_id)
                                    {
                                        target_neuron.process_spike_event(event, weight);
                                    }
                                }
                            }
                        }
                        if let Some(neuron) = neurons_guard.get_mut(&firing_neuron_id) {
                            neuron.apply_stdp(&spikes); // Apply learning rule
                        }
                    }
                }
            }
        }
        info!(
            "QP {}: Total fires over {} cycles: {}",
            self.id.0, self.config.snn_processing_cycles, total_fires
        );
        
        // Determine if an anomaly or pattern was detected based on firing activity.
        let pattern_detected = total_fires
            > (self.config.snn_quantum_pocket_default_neurons as usize
                * self.config.snn_processing_cycles as usize
                / 4); // Example threshold for pattern detection
        let anomaly_detected =
            total_fires < (self.config.snn_quantum_pocket_default_neurons as usize / 2); // Example threshold for anomaly detection
        
        QPSnapshot {
            anomaly_detected,
            pattern_detected,
        }
    }
}