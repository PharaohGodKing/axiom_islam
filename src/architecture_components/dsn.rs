//! src/architecture_components/dsn.rs
//! Defines the Deep Semantic Network (DSN).

use bincode::{Decode, Encode}; // Ensure Encode and Decode are imported for serialization
use crate::architecture_components::topical_bnn::{BNNOutput, TopicalBNN};
use crate::config::SystemConfig;
use crate::data_models::{ExternalDataPoint, SerializableUuid, DSNOutput}; // Correctly importing SerializableUuid and DSNOutput
use log::{info, warn};
use serde::{Deserialize, Serialize}; // Ensure Serialize and Deserialize are imported
use uuid::Uuid;

/// Represents a Deep Semantic Network (DSN) within the Axiom Islam architecture.
///
/// A DSN is responsible for processing data related to a specific semantic domain
/// and contains multiple Topical Bayesian Neural Networks (TopicalBNNs) for detailed analysis.
#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)] // Added Encode and Decode derives
pub struct DSN {
    // `id` is now correctly using the `SerializableUuid` wrapper.
    pub id: SerializableUuid,
    pub topical_bnns: Vec<TopicalBNN>,
    pub config: SystemConfig,
}

impl DSN {
    /// Creates a new Deep Semantic Network (DSN) instance.
    ///
    /// Initializes with a unique ID and a collection of TopicalBNNs
    /// based on the provided system configuration (`bnns_per_dsn`).
    pub fn new(config: SystemConfig) -> Self {
        let topical_bnns = (0..config.bnns_per_dsn)
            .map(|_| TopicalBNN::new(config.clone())) // TopicalBNN::new should take config
            .collect();
        DSN {
            // Correctly wrap the `Uuid::new_v4()` into `SerializableUuid`.
            id: SerializableUuid(Uuid::new_v4()),
            topical_bnns,
            config,
        }
    }

    /// Re-initializes the configuration for the DSN and all its contained TopicalBNNs.
    ///
    /// This is useful for dynamically updating system parameters without
    /// recreating the entire DSN structure, propagating changes down the hierarchy.
    pub fn re_init_config(&mut self, config: SystemConfig) {
        self.config = config.clone(); // Update DSN's own config
        for bnn in &mut self.topical_bnns {
            bnn.re_init_config(config.clone()); // Propagate config to contained BNNs
        }
    }

    /// Processes an external data point through all TopicalBNNs within this DSN.
    ///
    /// It aggregates anomaly ratios and pattern ratios from each BNN to provide an overall assessment
    /// of the data's deviation from established patterns within this semantic domain.
    /// It returns a `DSNOutput` summarizing the insights.
    ///
    /// # Arguments
    /// * `data` - The `ExternalDataPoint` to be processed.
    ///
    /// # Returns
    /// A `DSNOutput` containing an aggregated insight and the count of supporting BNNs.
    pub async fn process_data(&mut self, data: ExternalDataPoint) -> DSNOutput {
        let mut total_anomaly_ratio = 0.0;
        let mut total_pattern_ratio = 0.0; 
        let mut supporting_bnns_count_from_bnns = 0; 
        let mut bnn_analysis_summaries: Vec<String> = Vec::new();

        for bnn in &mut self.topical_bnns {
            let result: BNNOutput = bnn.process_data(data.clone()).await;
            total_anomaly_ratio += result.anomaly_ratio;
            total_pattern_ratio += result.pattern_ratio; 
            
            let bnn_summary = format!("BNN {}: Anomaly {:.2}, Pattern {:.2}", result.bnn_id.0, result.anomaly_ratio, result.pattern_ratio);
            bnn_analysis_summaries.push(bnn_summary);

            if result.pattern_ratio > self.config.bnn_consensus_threshold { 
                supporting_bnns_count_from_bnns += 1;
            }
        }
        
        let avg_anomaly_ratio = if !self.topical_bnns.is_empty() {
            total_anomaly_ratio / self.topical_bnns.len() as f64
        } else {
            0.0
        };
        let avg_pattern_ratio = if !self.topical_bnns.is_empty() {
            total_pattern_ratio / self.topical_bnns.len() as f64
        } else {
            0.0
        };


        info!("DSN {}: Avg Anomaly Ratio {:.2}, Avg Pattern Ratio {:.2}", self.id.0, avg_anomaly_ratio, avg_pattern_ratio);

        // --- Enhanced DSN Insight Generation (Pattern Recognition) ---
        let insight_text = if avg_anomaly_ratio > self.config.dsn_pol_threshold && avg_pattern_ratio < 0.2 { 
            warn!("DSN {} detected significant anomaly pattern.", self.id.0);
            format!("High semantic anomaly detected (Ratio: {:.2}). Indicates potential novel pattern or deviation from established data structures relevant to this domain.", avg_anomaly_ratio)
        } else if avg_pattern_ratio > self.config.dsn_pol_threshold && avg_anomaly_ratio < 0.3 { 
            format!("Strong semantic coherence observed (Pattern Ratio: {:.2}). Data aligns well with established patterns within this domain, indicating high relevance and understanding.", avg_pattern_ratio)
        } else if avg_pattern_ratio > 0.5 && avg_anomaly_ratio > 0.5 {
            format!("Mixed semantic signal (Pattern: {:.2}, Anomaly: {:.2}). Contains elements of known patterns but also significant novel or conflicting data. Requires deeper KSL synthesis.", avg_pattern_ratio, avg_anomaly_ratio)
        }
        else {
            format!("Moderate semantic relevance detected (Anomaly: {:.2}, Pattern: {:.2}). Further contextual analysis at KSL level is recommended.", avg_anomaly_ratio, avg_pattern_ratio)
        };
        // --- END Enhanced DSN Insight Generation ---

        // Combine insights for a more comprehensive DSNOutput
        let final_insight = format!(
            "{}\nSupporting BNNs: {}/{}. BNN Analysis: [{}]",
            insight_text,
            supporting_bnns_count_from_bnns, 
            self.topical_bnns.len(),
            bnn_analysis_summaries.join("; ")
        );

        DSNOutput {
            dsn_id: self.id.clone(),
            insight: final_insight,
            supporting_bnns: supporting_bnns_count_from_bnns, 
        }
    }
}