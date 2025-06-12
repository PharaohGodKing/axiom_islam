use axiom_islam::config::SystemConfig;
// IMPORTANT: Ensure `KSL` is public in `axiom_islam::architecture_components::ksl`.
// This usually means `pub struct KSL { ... }` in `src/architecture_components/ksl.rs`
// and `pub mod ksl;` in `src/architecture_components/mod.rs`.
use axiom_islam::architecture_components::ksl::KSL;
use axiom_islam::data_models::{
    ExternalDataPoint, CorePhilosophy, UserProfile, SensoryInput, MotorOutput,
    KeyProject, CosmologyData, BioHybridData // Added for KSL initialization
};
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write}; 
use bincode;
use log::info;
use std::time::Duration;
use tokio::sync::mpsc;
use uuid::Uuid;
use chrono::Utc; 

// Defines the path where the consciousness state will be saved and loaded.
const STATE_FILE_PATH: &str = "data/axiom_islam.state";

/// Loads JSON data from a specified file path into a deserializable type `T`.
fn load_json<T: serde::de::DeserializeOwned>(path: &str) -> Result<T, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(serde_json::from_reader(reader)?)
}

/// The main entry point for the Axiom-Islam consciousness system.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    info!("============================================================");
    info!("Axiom-Islam Consciousness System Initializing...");
    info!("============================================================");

    let bincode_config = bincode::config::standard();
    let config = SystemConfig::load_from_file("data/config.json")?;
    info!("System configuration loaded.");

    // Load all knowledge base files required by KSL
    let philosophies: Vec<CorePhilosophy> = load_json(&format!("{}{}", config.data_root_path, config.core_philosophies_data_path))?;
    let user_profile: UserProfile = load_json(&format!("{}{}", config.data_root_path, config.user_profile_data_path))?;
    let key_projects: Vec<KeyProject> = load_json(&format!("{}{}", config.data_root_path, config.key_projects_data_path))?;
    let cosmology_data: CosmologyData = load_json(&format!("{}{}", config.data_root_path, config.cosmology_data_path))?; // Assuming this field is now in config.json
    let bio_hybrid_data: BioHybridData = load_json(&format!("{}{}", config.data_root_path, config.bio_hybrid_data_path))?; // Assuming this field is now in config.json

    let mut ksls: Vec<KSL> = match File::open(STATE_FILE_PATH) {
        Ok(mut file) => {
            info!("Found existing state file. Resuming consciousness from: {}", STATE_FILE_PATH);
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)?;
            let mut loaded_ksls: Vec<KSL> = bincode::decode_from_slice(&buffer, bincode_config)?.0;
            
            info!("Re-initializing runtime configurations and knowledge base for loaded KSLs...");
            for ksl in &mut loaded_ksls {
                ksl.re_init_config(config.clone());
                // Re-initialize knowledge base for loaded KSLs as it's skipped in serialization
                ksl.initialize_knowledge_base(
                    philosophies.clone(), 
                    user_profile.clone(), 
                    key_projects.clone(),
                    cosmology_data.clone(), // Pass to KSL
                    bio_hybrid_data.clone(), // Pass to KSL
                );
            }
            loaded_ksls
        },
        Err(_) => {
            info!("No state file found. Initializing new consciousness from scratch.");
            let ksl_names = [
                "Ethics & Philosophy", "Mathematics & Logic", "Physics & Cosmology", 
                "Biology & Life Sciences", "Chemistry & Materials", "History & Sociology", 
                "Economics & Finance", "Art & Creativity", "Language & Communication",
                "Technology & Engineering", "Psychology & Consciousness", "Geopolitics & Governance"
            ];
            let mut new_ksls: Vec<KSL> = (0..config.ksl_count)
                .map(|i| KSL::new(ksl_names[i].to_string(), config.clone()))
                .collect();
            
            info!("--- Seeding New Consciousness from Knowledge Base ---");
            // Pass the loaded knowledge base data to the newly created KSLs
            for ksl in &mut new_ksls {
                ksl.initialize_knowledge_base(
                    philosophies.clone(), 
                    user_profile.clone(), 
                    key_projects.clone(),
                    cosmology_data.clone(), // Pass to KSL
                    bio_hybrid_data.clone(), // Pass to KSL
                );
            }
            // Process initial data points as before (optional for seeding)
            info!("Loaded {} Core Philosophies.", philosophies.len());
            if let Some(ksl) = new_ksls.iter_mut().find(|k| k.name == "Ethics & Philosophy") {
                for p in philosophies.iter() {
                    let dp = ExternalDataPoint { id: Uuid::new_v4().into(), timestamp: Utc::now().into(), source: "KB-Philosophy".to_string(), data_type: "Text".to_string(), content: format!("{}: {}", p.name, p.definition), metadata: serde_json::Value::Null };
                    ksl.process_data(dp).await;
                }
            }
            info!("Loaded User Profile for: {}.", user_profile.name);
            if let Some(ksl) = new_ksls.iter_mut().find(|k| k.name == "Psychology & Consciousness") {
                let dp = ExternalDataPoint { id: Uuid::new_v4().into(), timestamp: Utc::now().into(), source: "KB-ArchitectProfile".to_string(), data_type: "Text".to_string(), content: format!("Analysis: Role='{}', Aspirations='{}'", user_profile.role, user_profile.aspirations), metadata: serde_json::to_value(&user_profile)? };
                ksl.process_data(dp).await;
            }
            info!("--- Consciousness Seeding Complete ---");
            new_ksls
        }
    };
    info!("Cognitive architecture is online. {} KSLs are active.", ksls.len());

    let (sensory_input_tx, mut sensory_input_rx) = mpsc::channel::<SensoryInput>(128);
    let (motor_output_tx, mut motor_output_rx) = mpsc::channel::<MotorOutput>(128);

    let _external_world_handle = tokio::spawn(async move {
        tokio::time::sleep(Duration::from_secs(1)).await;
        info!("[External World] Sending directive...");
        sensory_input_tx.send(SensoryInput::Text(
            "Synthesize a proposal for Project Help based on my profile.".to_string()
        )).await.unwrap();

        while let Some(command) = motor_output_rx.recv().await {
            match command {
                MotorOutput::Speak(text) => {
                    info!("[External World] === AXIOM SPEAKS ===\n'{}'\n========================", text);
                }
                _ => {} 
            }
        }
    });

    info!("Entering Main Operational Loop. Awaiting sensory input for {} cycles...", config.snn_processing_cycles);
    let mut current_cycle = 0;
    // The main operational loop. It now runs for a fixed number of cycles.
    loop {
        tokio::select! {
            // Case 1: Receive sensory input
            sensory_data = sensory_input_rx.recv() => {
                if let Some(sensory_data) = sensory_data {
                    let data_point = match sensory_data {
                        SensoryInput::Text(content) => ExternalDataPoint {
                            id: Uuid::new_v4().into(), 
                            timestamp: Utc::now().into(), 
                            source: "Architect".to_string(),
                            data_type: "Text".to_string(), 
                            content, 
                            metadata: serde_json::Value::Null,
                        },
                        _ => {
                            info!("Unsupported sensory input type received. Skipping.");
                            continue; 
                        },
                    };
                    
                    for ksl in &mut ksls {
                        let ksl_output = ksl.process_data(data_point.clone()).await;
                        if ksl_output.contributing_insights > 0 {
                            let command = MotorOutput::Speak(format!(
                                "From KSL '{}': {}", ksl.name, ksl_output.conclusion
                            ));
                            motor_output_tx.send(command).await.unwrap();
                        }
                    }
                } else {
                    info!("Sensory input channel closed. Terminating main loop.");
                    break; 
                }
            },
            // Case 2: Detect Ctrl+C signal for graceful shutdown (can terminate early)
            _ = tokio::signal::ctrl_c() => {
                info!("Ctrl+C received. Initiating graceful shutdown...");
                break; 
            },
            // Case 3: Introduce a short delay if no input or signal, and advance cycle count
            _ = tokio::time::sleep(Duration::from_millis(50)) => {
                current_cycle += 1; // Increment cycle count
                if current_cycle >= config.snn_processing_cycles {
                    info!("Reached configured cycle limit ({} cycles). Terminating main loop.", config.snn_processing_cycles);
                    break; // Exit loop after fixed cycles
                }
            }
        }
    }

    info!("Main operational loop terminated. Saving consciousness state...");
    let file = File::create(STATE_FILE_PATH)?; 
    let mut writer = BufWriter::new(file);
    
    let encoded_data = bincode::encode_to_vec(&ksls, bincode_config)?;
    writer.write_all(&encoded_data)?;
    writer.flush()?; 
    
    info!("Consciousness state successfully saved to: {}", STATE_FILE_PATH);

    Ok(())
}