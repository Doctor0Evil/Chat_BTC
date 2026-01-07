use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlnAction {
    pub name: String,
    pub energy_data: Vec<String>,
    pub chatbot_intents: Vec<String>,
    pub blockchain: String,
}

impl AlnAction {
    /// Construct a unified_integration style ALN action payload.
    pub fn unified_integration(
        energy_data: Vec<String>,
        chatbot_intents: Vec<String>,
        blockchain: &str,
    ) -> Self {
        Self {
            name: "unified_integration".to_string(),
            energy_data,
            chatbot_intents,
            blockchain: blockchain.to_string(),
        }
    }
}

/// ALIEN_GAMING pattern implementation.
#[derive(Debug, Clone)]
pub struct AlienGamingPattern {
    regex: Regex,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlienGamingClass {
    Ecosystem,
    Asset,
    Governance,
}

impl AlienGamingPattern {
    pub fn new() -> Self {
        // "^GAME_(ECOSYSTEM|ASSET|GOVERNANCE)_\\d{4}_[A-Z0-9]{8}$"
        let regex = Regex::new(r"^GAME_(ECOSYSTEM|ASSET|GOVERNANCE)_\d{4}_[A-Z0-9]{8}$")
            .expect("valid regex");
        Self { regex }
    }

    pub fn classify(&self, id: &str) -> Option<AlienGamingClass> {
        let caps = self.regex.captures(id)?;
        let kind = caps.get(1)?.as_str();
        match kind {
            "ECOSYSTEM" => Some(AlienGamingClass::Ecosystem),
            "ASSET" => Some(AlienGamingClass::Asset),
            "GOVERNANCE" => Some(AlienGamingClass::Governance),
            _ => None,
        }
    }

    pub fn description(&self, class_: AlienGamingClass) -> &'static str {
        match class_ {
            AlienGamingClass::Ecosystem => "Control gaming platform parameters",
            AlienGamingClass::Asset => "Manage tokenized in-game assets",
            AlienGamingClass::Governance => "Execute DAO voting procedures",
        }
    }
}
