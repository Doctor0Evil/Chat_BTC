use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChainKind {
    Ethereum,
    Solana,
    BitcoinL2,
    EnergyWeb,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyAsset {
    pub asset_id: String,
    pub watts: f64,
    pub owner_address: String,
    pub nft_token_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatbotIntent {
    pub intent_id: String,
    pub text: String,
    pub source_platform: String, // grok | mistral | chatgpt | etc.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedIntegrationInput {
    pub energy_data: Vec<String>,
    pub chatbot_intents: Vec<String>,
    pub blockchain: ChainKind,
}
