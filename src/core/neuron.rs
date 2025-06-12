//! src/core/neuron.rs
//! Defines the Leaky Integrate-and-Fire (LIF) Neuron model.

use bincode::{Decode, Encode}; // Ensure Encode and Decode are imported
use crate::config::SystemConfig;
use crate::data_models::{SerializableDateTimeUtc, SerializableUuid, SpikeEvent}; // Correct import
use chrono::Utc;
use log::debug;
use rand::Rng; // Import Rng trait for random number generation
use rand::rngs::ThreadRng; // Specific import for ThreadRng
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Represents a Leaky Integrate-and-Fire (LIF) Neuron, a fundamental building block
/// of spiking neural networks.
///
/// It models the behavior of biological neurons, accumulating input, potentially firing
/// an action potential (spike), and then resetting.
#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)] // Added Encode and Decode derives
pub struct LIFNeuron {
    pub id: SerializableUuid,
    pub membrane_potential: f64,
    pub threshold: f64,
    pub leak_rate: f64,
    pub reset_potential: f64,
    pub refractory_countdown: u32,
    pub refractory_cycles: u32,
    pub connections: HashMap<SerializableUuid, f64>,
    pub last_spike_time: Option<SerializableDateTimeUtc>,
    pub learning_rate: f64,
}

impl LIFNeuron {
    /// Creates a new LIFNeuron instance with parameters initialized from the `SystemConfig`.
    ///
    /// # Arguments
    /// * `config` - A reference to the system's configuration.
    ///
    /// # Returns
    /// A new `LIFNeuron` instance.
    pub fn new(config: &SystemConfig) -> Self {
        LIFNeuron {
            id: SerializableUuid(Uuid::new_v4()),
            membrane_potential: config.lif_neuron_reset_potential,
            threshold: config.lif_neuron_threshold,
            leak_rate: config.lif_neuron_leak_rate,
            reset_potential: config.lif_neuron_reset_potential,
            refractory_countdown: 0,
            refractory_cycles: config.lif_neuron_refractory_cycles,
            connections: HashMap::new(),
            last_spike_time: None,
            learning_rate: config.default_learning_rate,
        }
    }

    /// Forms a synaptic connection from this neuron to a target neuron.
    ///
    /// Assigns an initial random weight to the connection.
    ///
    /// # Arguments
    /// * `target_neuron_id` - The `SerializableUuid` of the neuron to connect to.
    #[allow(deprecated)] // Allow deprecated for rand::thread_rng if Cargo.toml has an older rand version
    pub fn form_connection(&mut self, target_neuron_id: SerializableUuid) {
        let mut rng: ThreadRng = rand::thread_rng();
        #[allow(deprecated)] // Allow deprecated for rand::Rng::gen_range
        let initial_weight = rng.gen_range(0.01..0.1);
        self.connections.insert(target_neuron_id, initial_weight);
    }

    /// Processes an incoming spike event, updating the neuron's membrane potential.
    ///
    /// The potential increases based on the spike's value and the connection weight,
    /// unless the neuron is in its refractory period.
    ///
    /// # Arguments
    /// * `event` - A reference to the `SpikeEvent` received.
    /// * `weight` - The synaptic weight of the connection through which the spike arrived.
    pub fn process_spike_event(&mut self, event: &SpikeEvent, weight: f64) {
        if self.refractory_countdown > 0 {
            return;
        }
        self.membrane_potential += event.value * weight;
    }

    /// Updates the neuron's state and checks if it fires an action potential.
    ///
    /// Applies the leak rate, resets potential if it falls too low, and checks if
    /// the membrane potential has reached the firing threshold. If it fires,
    /// it enters a refractory period.
    ///
    /// # Returns
    /// `true` if the neuron fired, `false` otherwise.
    pub fn update_and_check_fire(&mut self) -> bool {
        if self.refractory_countdown > 0 {
            self.refractory_countdown -= 1;
            return false;
        }
        self.membrane_potential -= self.leak_rate;
        if self.membrane_potential < self.reset_potential {
            self.membrane_potential = self.reset_potential;
        }
        if self.membrane_potential >= self.threshold {
            debug!("Neural Unit {} FIRED!", self.id.0);
            self.membrane_potential = self.reset_potential;
            self.refractory_countdown = self.refractory_cycles;
            self.last_spike_time = Some(SerializableDateTimeUtc(Utc::now()));
            true
        } else {
            false
        }
    }

    /// Applies Spike-Timing Dependent Plasticity (STDP) to adjust synaptic weights.
    ///
    /// Weights are strengthened if presynaptic spikes occur just before postsynaptic
    /// firing, and weakened if they occur just after.
    ///
    /// # Arguments
    /// * `recent_input_events` - A slice of recent `SpikeEvent`s that influenced this neuron.
    pub fn apply_stdp(&mut self, recent_input_events: &[SpikeEvent]) {
        if let Some(post_synaptic_fire_time) = &self.last_spike_time {
            for input_event in recent_input_events {
                if let Some(weight) = self.connections.get_mut(&input_event.presynaptic_neuron_id)
                {
                    // Calculate time difference between pre-synaptic spike and post-synaptic fire.
                    let dt_ms = post_synaptic_fire_time
                        .0 // Access the inner DateTime<Utc>
                        .signed_duration_since(input_event.timestamp.0) // Access the inner DateTime<Utc>
                        .num_milliseconds();
                    
                    // Apply STDP rule: strengthen if pre-synaptic comes before post-synaptic (within window)
                    if dt_ms > 0 && dt_ms < 20 { // Presynaptic before postsynaptic
                        let delta = self.learning_rate * (1.0 / (dt_ms as f64).max(0.1));
                        *weight += delta;
                    } 
                    // Apply STDP rule: weaken if presynaptic comes after postsynaptic (within window)
                    else if dt_ms < 0 && dt_ms > -20 { // Presynaptic after postsynaptic
                        let delta = self.learning_rate * (1.0 / (dt_ms.abs() as f64).max(0.1));
                        *weight -= delta;
                    }
                    // Clamp weight to ensure it stays within valid bounds
                    *weight = weight.clamp(0.0, 1.0);
                }
            }
        }
    }
}