//! src/data_models.rs
//! Defines shared data structures used across the Axiom Islam system.

use bincode::{
    de::{BorrowDecoder, Decoder},
    enc::{Encoder, self}, // Ensure enc::self is imported for enc::Encode
    error::{DecodeError, EncodeError},
    BorrowDecode, Decode, Encode,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;
use serde_json::Value; // Import Value for explicit type hinting

// --- Wrapper for Uuid to implement bincode traits ---
// Removed #[derive(Encode, Decode)] as manual impls are provided below.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)] 
pub struct SerializableUuid(pub Uuid);

impl Encode for SerializableUuid {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.0.as_bytes().encode(encoder)
    }
}

impl<C> Decode<C> for SerializableUuid {
    fn decode<D: Decoder<Context = C>>(decoder: &mut D) -> Result<Self, DecodeError> {
        let bytes: [u8; 16] = Decode::decode(decoder)?;
        Ok(SerializableUuid(Uuid::from_bytes(bytes)))
    }
}

impl<'de, C: Sized> BorrowDecode<'de, C> for SerializableUuid {
    fn borrow_decode<D: BorrowDecoder<'de, Context = C>>(decoder: &mut D) -> Result<Self, DecodeError> {
        let bytes: [u8; 16] = BorrowDecode::borrow_decode(decoder)?;
        Ok(SerializableUuid(Uuid::from_bytes(bytes)))
    }
}


// --- Wrapper for DateTime<Utc> to implement bincode traits ---
// Removed #[derive(Encode, Decode)] as manual impls are provided below.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)] 
pub struct SerializableDateTimeUtc(pub DateTime<Utc>);

impl Encode for SerializableDateTimeUtc {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.0.timestamp_nanos_opt().encode(encoder)
    }
}

impl<C> Decode<C> for SerializableDateTimeUtc {
    fn decode<D: Decoder<Context = C>>(decoder: &mut D) -> Result<Self, DecodeError> {
        let nanos: i64 = Decode::decode(decoder)?;
        Ok(SerializableDateTimeUtc(DateTime::<Utc>::from_timestamp_nanos(nanos)))
    }
}

impl<'de, C: Sized> BorrowDecode<'de, C> for SerializableDateTimeUtc {
    fn borrow_decode<D: BorrowDecoder<'de, Context = C>>(decoder: &mut D) -> Result<Self, DecodeError> {
        let nanos: i64 = BorrowDecode::borrow_decode(decoder)?;
        Ok(SerializableDateTimeUtc(DateTime::<Utc>::from_timestamp_nanos(nanos)))
    }
}


// --- Core Data Models ---

// Manual Encode/Decode for ExternalDataPoint to skip `metadata: Value`
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct ExternalDataPoint {
    pub id: SerializableUuid,
    pub timestamp: SerializableDateTimeUtc,
    pub source: String,
    pub data_type: String,
    pub content: String,
    #[serde(default)] 
    pub metadata: Value, // This field is skipped for bincode via manual impl
}

impl Encode for ExternalDataPoint {
    fn encode<E: enc::Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.id.encode(encoder)?;
        self.timestamp.encode(encoder)?;
        self.source.encode(encoder)?;
        self.data_type.encode(encoder)?;
        self.content.encode(encoder)?;
        // Skip metadata for bincode serialization
        Ok(())
    }
}

impl<C> Decode<C> for ExternalDataPoint {
    fn decode<D: Decoder<Context = C>>(decoder: &mut D) -> Result<Self, DecodeError> {
        let id = Decode::decode(decoder)?;
        let timestamp = Decode::decode(decoder)?;
        let source = Decode::decode(decoder)?;
        let data_type = Decode::decode(decoder)?;
        let content = Decode::decode(decoder)?;
        // When decoding, provide a default for metadata as it was skipped
        Ok(ExternalDataPoint { id, timestamp, source, data_type, content, metadata: Value::Null })
    }
}


// NOTE: BNNOutput structure is defined in `topical_bnn.rs`

// DSNOutput structure: Must match what is consistently used by DSN and KSL.
#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct DSNOutput {
    pub dsn_id: SerializableUuid,
    pub insight: String,
    pub supporting_bnns: usize, 
}

// SpikeEvent is used by neuron.rs and snn_quantum_pocket.rs
#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct SpikeEvent {
    pub presynaptic_neuron_id: SerializableUuid,
    pub timestamp: SerializableDateTimeUtc,
    pub value: f64,
}

// NOTE: KSLOutput structure is defined in `ksl.rs`, NOT here.


// CorePhilosophy: Needs Encode/Decode
#[derive(Debug, Clone, Serialize, Deserialize, Default, Encode, Decode)] 
pub struct CorePhilosophy {
    pub name: String,
    pub definition: String,
    pub framework_integration: String,
    pub hint_for_axiom: String,
}

// KeyProject: Needs Encode/Decode
#[derive(Debug, Clone, Serialize, Deserialize, Default, Encode, Decode)] 
pub struct KeyProject {
    pub name: String, 
    pub nature: String,
    pub core_purpose: String,
    pub mission_alignment: String,
    #[serde(default)] 
    pub operational_link: Option<Vec<String>> 
}
// UserProfile: Needs Encode/Decode, but contains serde_json::Value.
// Manual Encode/Decode to skip `technology_ecosystem` and `education` (Vec<Value>).
#[derive(Debug, Clone, Serialize, Deserialize, Default)] // Removed Encode/Decode
pub struct UserProfile {
    pub name: String,
    pub role: String,
    pub core_mission: String,
    #[serde(default)]
    pub personal_principles: Vec<String>,
    #[serde(default)]
    pub birth_info: Option<HashMap<String, String>>,
    #[serde(default)]
    pub education: Vec<Value>, // Contains Value
    #[serde(default)]
    pub skills: Vec<String>,
    #[serde(default)]
    pub interests: Vec<String>,
    #[serde(default)]
    pub favorite_colors: Vec<String>,
    #[serde(default)]
    pub technology_ecosystem: Value, // Contains Value
    #[serde(default)] // ADDED THIS LINE: Allows 'aspirations' to be missing in JSON
    pub aspirations: String,
    #[serde(default)]
    pub strategic_goals: Vec<String>,
    #[serde(default)]
    pub ai_view: String,
    #[serde(default)]
    pub world_peace_mission: String
}

impl Encode for UserProfile {
    fn encode<E: enc::Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.name.encode(encoder)?;
        self.role.encode(encoder)?;
        self.core_mission.encode(encoder)?;
        self.personal_principles.encode(encoder)?;
        self.birth_info.encode(encoder)?;
        // Skip education and technology_ecosystem as before
        self.skills.encode(encoder)?;
        self.interests.encode(encoder)?;
        self.favorite_colors.encode(encoder)?;
        self.aspirations.encode(encoder)?; // Keep this encoded if present
        self.strategic_goals.encode(encoder)?;
        self.ai_view.encode(encoder)?;
        self.world_peace_mission.encode(encoder)?;
        Ok(())
    }
}

impl<C> Decode<C> for UserProfile {
    fn decode<D: Decoder<Context = C>>(decoder: &mut D) -> Result<Self, DecodeError> {
        let name = Decode::decode(decoder)?;
        let role = Decode::decode(decoder)?;
        let core_mission = Decode::decode(decoder)?;
        let personal_principles = Decode::decode(decoder)?;
        let birth_info = Decode::decode(decoder)?;
        // Provide defaults for skipped fields
        let education = Default::default();
        let skills = Decode::decode(decoder)?;
        let interests = Decode::decode(decoder)?;
        let favorite_colors = Decode::decode(decoder)?;
        let technology_ecosystem = Value::Null; // Provide default for skipped field
        let aspirations = Decode::decode(decoder)?; // Still decode if it's there
        let strategic_goals = Decode::decode(decoder)?;
        let ai_view = Decode::decode(decoder)?;
        let world_peace_mission = Decode::decode(decoder)?;

        Ok(UserProfile {
            name, role, core_mission, personal_principles, birth_info, education,
            skills, interests, favorite_colors, technology_ecosystem, aspirations,
            strategic_goals, ai_view, world_peace_mission
        })
    }
}

// CosmologyData: Needs Encode/Decode, but contains serde_json::Value.
// Manual Encode/Decode to skip all its fields.
#[derive(Debug, Clone, Serialize, Deserialize, Default)] // Removed Encode/Decode
pub struct CosmologyData {
    pub cosmology: Value, // Contains Value
    pub ordered_placement_of_beings: Value, // Contains Value
    pub ancient_wisdom_perceptual_matrix: Value, // Contains Value
}

impl Encode for CosmologyData {
    fn encode<E: enc::Encoder>(&self, _encoder: &mut E) -> Result<(), EncodeError> {
        // Skip all fields for bincode serialization
        Ok(())
    }
}

impl<C> Decode<C> for CosmologyData {
    fn decode<D: Decoder<Context = C>>(_decoder: &mut D) -> Result<Self, DecodeError> {
        // Provide defaults for skipped fields when decoding
        Ok(CosmologyData {
            cosmology: Value::Null,
            ordered_placement_of_beings: Value::Null,
            ancient_wisdom_perceptual_matrix: Value::Null,
        })
    }
}


// BioHybridData: Needs Encode/Decode, but contains many serde_json::Value fields.
// Manual Encode/Decode to skip all its fields.
#[derive(Debug, Clone, Serialize, Deserialize, Default)] // Removed Encode/Decode
pub struct BioHybridData {
    pub vision: Value,
    pub seven_core_bio_hybrid_concepts: Value,
    pub physical_mechanical_systems: Value,
    pub cognitive_sensory_systems: Value,
    pub biological_regenerative_functions: Value,
    pub ai_model_fine_tuning_directives: Value,
}

impl Encode for BioHybridData {
    fn encode<E: enc::Encoder>(&self, _encoder: &mut E) -> Result<(), EncodeError> {
        // Skip all fields for bincode serialization
        Ok(())
    }
}

impl<C> Decode<C> for BioHybridData {
    fn decode<D: Decoder<Context = C>>(_decoder: &mut D) -> Result<Self, DecodeError> {
        // Provide defaults for skipped fields when decoding
        Ok(BioHybridData {
            vision: Value::Null,
            seven_core_bio_hybrid_concepts: Value::Null,
            physical_mechanical_systems: Value::Null,
            cognitive_sensory_systems: Value::Null,
            biological_regenerative_functions: Value::Null,
            ai_model_fine_tuning_directives: Value::Null,
        })
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub enum SensoryInput {
    Text(String),
    Vision {
        format: String,
        resolution: (u32, u32),
        data: Vec<u8>,
    },
    Audio {
        format: String,
        sample_rate: u32,
        data: Vec<u8>,
    },
    Telemetry {
        source_device: String,
        data: HashMap<String, f64>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MotorOutput {
    Speak(String),
    ExecuteAction {
        action_name: String,
        parameters: HashMap<String, Value>, // Value is fine here as this enum is not bincode-derived
    },
    ControlRoboticComponent {
        component: String,
        command: String,
        target: (f64, f64, f64),
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MotorControlCommand {
    pub command: String,
    pub parameters: HashMap<String, Value>, // Value is fine here
}

#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct MotorControlFeedback {
    pub status: String,
    pub timestamp: SerializableDateTimeUtc,
}

#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct MotorControlResponse {
    pub status: String,
    pub feedback: MotorControlFeedback,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializedNeuron {
    pub id: SerializableUuid,
    pub name: String,
    pub description: String,
    pub parameters: HashMap<String, Value>, // Value is fine here
    pub connections: Vec<SerializableUuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializedNeuralNetwork {
    pub id: SerializableUuid,
    pub name: String,
    pub description: String,
    pub neurons: Vec<SerializedNeuron>,
    pub connections: HashMap<SerializableUuid, Vec<SerializableUuid>>,
}

impl From<Uuid> for SerializableUuid {
    fn from(uuid: Uuid) -> Self {
        SerializableUuid(uuid)
    }
}
impl From<SerializableUuid> for Uuid {
    fn from(s_uuid: SerializableUuid) -> Self {
        s_uuid.0
    }
}
impl From<DateTime<Utc>> for SerializableDateTimeUtc {
    fn from(dt: DateTime<Utc>) -> Self {
        SerializableDateTimeUtc(dt)
    }
}
impl From<SerializableDateTimeUtc> for DateTime<Utc> {
    fn from(serializable: SerializableDateTimeUtc) -> Self {
        serializable.0
    }
}

// --- Global Utility for Keyword Extraction ---
/// Extracts keywords from a given text, filtering out common stop words and punctuation.
///
/// Converts text to lowercase and splits by whitespace.
///
/// # Arguments
/// * `text` - The input string from which to extract keywords.
///
/// # Returns
/// A `Vec<String>` containing the extracted keywords.
pub fn extract_keywords(text: &str) -> Vec<String> {
    let stop_words: HashSet<&str> = [
        "a", "an", "the", "and", "or", "but", "is", "are", "was", "were", "be", "been", "being",
        "to", "of", "in", "on", "at", "for", "with", "as", "by", "from", "about", "what", "how",
        "who", "when", "where", "why", "can", "will", "my", "your", "his", "her", "its", "our",
        "their", "this", "that", "these", "those", "it", "he", "she", "we", "they", "i", "you",
        "me", "him", "us", "them", "which", "whom", "whose", "do", "don", "does", "doesn", "did",
        "didn", "has", "hasn", "have", "haven", "had", "hadn", "not", "no", "yes", "so", "than",
        "then", "just", "now", "only", "very", "too", "much", "more", "most", "less", "least",
        "many", "few", "some", "any", "all", "none", "every", "each", "both", "either", "neither",
        "own", "same", "such", "up", "down", "out", "off", "over", "under", "again", "further",
        "then", "once", "here", "there", "when", "where", "why", "how", "all", "any", "both", "each",
        "few", "more", "most", "other", "some", "such", "no", "nor", "not", "only", "own", "same",
        "so", "than", "too", "very", "s", "t", "can", "will", "just", "don", "should", "now",
    ]
    .iter()
    .cloned()
    .collect();

    text.to_lowercase()
        .replace(|c: char| c.is_ascii_punctuation(), " ")
        .split_whitespace()
        .filter(|word| word.len() > 2 && !stop_words.contains(word))
        .map(String::from)
        .collect()
}