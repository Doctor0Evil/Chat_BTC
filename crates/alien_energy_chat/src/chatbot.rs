use crate::types::ChatbotIntent;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatbotMetrics {
    pub throughput_tps: u32,
    pub latency_ms: u32,
    pub error_rate: f32,
}

impl ChatbotMetrics {
    pub fn default_from_analysis() -> Self {
        Self {
            throughput_tps: 3800,
            latency_ms: 28,
            error_rate: 0.0008,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatbotSyncTargets {
    pub grok: bool,
    pub mistral: bool,
    pub chatgpt: bool,
}

impl ChatbotSyncTargets {
    pub fn default_sync_all() -> Self {
        Self {
            grok: true,
            mistral: true,
            chatgpt: true,
        }
    }

    pub fn as_list(&self) -> Vec<&'static str> {
        let mut v = Vec::new();
        if self.grok {
            v.push("grok");
        }
        if self.mistral {
            v.push("mistral");
        }
        if self.chatgpt {
            v.push("chatgpt");
        }
        v
    }
}

pub struct ChatbotEngine {
    pub metrics: ChatbotMetrics,
    pub sync_targets: ChatbotSyncTargets,
}

impl ChatbotEngine {
    pub fn new(sync_targets: ChatbotSyncTargets) -> Self {
        Self {
            metrics: ChatbotMetrics::default_from_analysis(),
            sync_targets,
        }
    }

    pub fn generate_flows(&self, intents: &[ChatbotIntent]) -> usize {
        intents.len()
    }
}
