//! src/architecture_components/topical_bnn.rs
//! Defines the Topical Bayesian Neural Network (TopicalBNN).

use bincode::{Decode, Encode}; // Import Encode and Decode for bincode serialization
use crate::config::SystemConfig;
use crate::core::neuron::LIFNeuron; // Assuming LIFNeuron is used within TopicalBNN
use crate::data_models::{ExternalDataPoint, SerializableUuid, extract_keywords}; // Import SerializableUuid and extract_keywords
use log::info;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::{HashMap, HashSet}; // Added HashSet for topic_keywords

/// Represents the output of a Topical Bayesian Neural Network (TopicalBNN).
/// This struct summarizes the BNN's analysis, including anomaly and pattern ratios.
#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct BNNOutput {
    pub bnn_id: SerializableUuid,
    pub anomaly_ratio: f64,
    pub pattern_ratio: f64, 
}

/// Represents a Topical Bayesian Neural Network (TopicalBNN).
///
/// A TopicalBNN focuses on processing data within a specific topic or context,
/// integrating spiking neural network principles for dynamic learning.
#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)] // Add Encode and Decode
pub struct TopicalBNN {
    pub id: SerializableUuid,
    pub topic: String, // E.g., "Economics", "Physics", "Emotion"
    pub neurons: HashMap<SerializableUuid, LIFNeuron>, // Assuming it contains neurons
    pub config: SystemConfig,
    pub topic_keywords: HashSet<String>, // NEW: Keywords relevant to this BNN's topic
}

impl TopicalBNN {
    /// Creates a new TopicalBNN instance with a given topic and system configuration.
    ///
    /// Initializes the BNN with a unique ID, a set of LIF neurons, and a set of
    /// topic-specific keywords for semantic processing.
    pub fn new(config: SystemConfig) -> Self {
        let id = SerializableUuid(Uuid::new_v4());
        let topic = "General".to_string(); // Default topic, can be specialized later
        let mut neurons = HashMap::new();
        // Initialize some neurons for the BNN
        for _ in 0..config.bnn_default_neurons { 
            let neuron = LIFNeuron::new(&config);
            neurons.insert(neuron.id.clone(), neuron);
        }

        // Initialize topic keywords based on the BNN's topic (simplified for now)
        let topic_keywords = match topic.as_str() {
            "Ethics & Philosophy" => ["peace", "love", "truth", "justice", "ethics", "moral", "principle", "understanding", "wisdom", "foundation", "unity"].iter().map(|s| s.to_string()).collect(),
            "Psychology & Consciousness" => ["consciousness", "mind", "soul", "spirit", "awareness", "identity", "psychology", "cognition", "empathy"].iter().map(|s| s.to_string()).collect(),
            "Technology & Engineering" => ["ai", "robotics", "bio-hybrid", "quantum", "engineering", "system", "tech", "nanodocs", "gemma", "platform"].iter().map(|s| s.to_string()).collect(),
            "Physics & Cosmology" => ["cosmology", "universe", "quantum", "actuality", "heavens", "origin", "duality", "attraction", "retraction", "yeshua", "lucifer", "anunnaki"].iter().map(|s| s.to_string()).collect(),
            "Economics & Finance" => ["business", "finance", "investment", "economy", "market", "wealth", "royalty", "subsidiary"].iter().map(|s| s.to_string()).collect(),
            "Art & Creativity" => ["art", "music", "create", "design", "expression", "visual", "audio"].iter().map(|s| s.to_string()).collect(),
            "History & Sociology" => ["history", "society", "culture", "sociology", "oppression", "narrative", "ancient", "civilization"].iter().map(|s| s.to_string()).collect(),
            // ... add more topics as needed, or a general fallback
            _ => HashSet::new(),
        };


        TopicalBNN {
            id,
            topic,
            neurons,
            config,
            topic_keywords, // Assign the new field
        }
    }

    /// Re-initializes the configuration for the TopicalBNN and its contained neurons.
    ///
    /// This method ensures that configuration changes are propagated throughout
    /// the BNN's sub-components.
    pub fn re_init_config(&mut self, config: SystemConfig) {
        self.config = config.clone();
        for neuron in self.neurons.values_mut() {
            neuron.threshold = config.lif_neuron_threshold;
            neuron.leak_rate = config.lif_neuron_leak_rate;
            neuron.reset_potential = config.lif_neuron_reset_potential;
            neuron.refractory_cycles = config.lif_neuron_refractory_cycles;
            neuron.learning_rate = config.default_learning_rate;
        }
    }

    /// Processes an external data point within the context of this TopicalBNN.
    ///
    /// This method now uses keyword matching to provide a more semantically relevant input
    /// to the neurons, generating `BNNOutput` based on the network's response to these
    /// semantic features. This enhances pattern recognition.
    ///
    /// # Arguments
    /// * `data` - The `ExternalDataPoint` to be processed.
    ///
    /// # Returns
    /// A `BNNOutput` summarizing the BNN's analysis based on anomaly and pattern ratios,
    /// which are now influenced by semantic relevance.
    pub async fn process_data(&mut self, data: ExternalDataPoint) -> BNNOutput {
        info!("TopicalBNN '{}' processing data for topic '{}'...", self.id.0, self.topic);

        let input_keywords = extract_keywords(&data.content);
        let mut matched_topic_keywords_count = 0;
        for keyword in &input_keywords {
            if self.topic_keywords.contains(keyword) {
                matched_topic_keywords_count += 1;
            }
        }

        // Calculate a more semantic input strength: combines content length AND keyword relevance.
        // A simple model: base strength from length + bonus for matched keywords.
        let base_strength = (data.content.len() as f64 * 0.01) * self.config.neuron_activity_multiplier;
        let keyword_strength = matched_topic_keywords_count as f64 * 0.1; // Each matched keyword adds some strength
        let total_input_strength = base_strength + keyword_strength;

        // Corrected: Renamed `fires_count` to `_fires_count` to silence unused variable warning.
        let mut _fires_count = 0; 
        let mut total_neurons_fired_at_least_once = 0;

        for (_neuron_id, neuron) in self.neurons.iter_mut() {
            neuron.membrane_potential += total_input_strength; // Use the semantic input strength
            if neuron.update_and_check_fire() {
                _fires_count += 1; // Still incrementing _fires_count
                total_neurons_fired_at_least_once += 1; // Count distinct neurons that fired
            }
        }

        let total_neurons = self.neurons.len();
        // Anomaly: high if very few neurons fire relative to expectation, or if input is completely alien.
        let anomaly_ratio = if total_neurons > 0 {
            1.0 - (total_neurons_fired_at_least_once as f64 / total_neurons as f64).clamp(0.0, 1.0)
        } else {
            0.0
        };
        
        // Pattern: high if a significant number of neurons fired (indicating a recognized pattern)
        let pattern_ratio = if total_neurons > 0 {
            (total_neurons_fired_at_least_once as f64 / total_neurons as f64).clamp(0.0, 1.0)
        } else {
            0.0
        };

        BNNOutput {
            bnn_id: self.id.clone(),
            anomaly_ratio,
            pattern_ratio,
        }
    }
}