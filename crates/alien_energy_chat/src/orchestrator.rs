use crate::aln::{AlnAction, AlienGamingPattern};
use crate::chatbot::{ChatbotEngine, ChatbotSyncTargets};
use crate::docker::DockerOrchestrator;
use crate::energy::{CrossChainConfig, EnergyBallEngine, EnergyTradingContractConfig};
use crate::types::{ChainKind, ChatbotIntent, EnergyAsset, UnifiedIntegrationInput};

#[derive(Debug)]
pub struct UnifiedIntegrationOrchestrator {
    pub docker: DockerOrchestrator,
    pub energy_engine: EnergyBallEngine,
    pub chatbot_engine: ChatbotEngine,
    pub aln_pattern: AlienGamingPattern,
}

impl UnifiedIntegrationOrchestrator {
    pub fn new(
        docker: DockerOrchestrator,
        chain: ChainKind,
    ) -> Self {
        let cross_chain = CrossChainConfig::recommended();
        let energy_engine =
            EnergyBallEngine::new(EnergyTradingContractConfig::new(chain, cross_chain));
        let chatbot_engine = ChatbotEngine::new(ChatbotSyncTargets::default_sync_all());
        let aln_pattern = AlienGamingPattern::new();
        Self {
            docker,
            energy_engine,
            chatbot_engine,
            aln_pattern,
        }
    }

    /// High-level ALN-style unified integration.
    ///
    /// Conceptually mirrors:
    /// @ACTION unified_integration { @INGEST, @DEPLOY, @GENERATE, @SYNC }
    pub fn execute_unified_integration(
        &self,
        input: UnifiedIntegrationInput,
    ) -> AlnAction {
        // 1) Ingest energy data (logical step; storage is handled by docker volume layer).
        let energy_data = input.energy_data;

        // 2) Deploy energy trading contract to chosen chain.
        self.energy_engine.deploy_contracts();

        // 3) Generate chatbot flows (prepare intents for multi-platform use).
        let intents: Vec<ChatbotIntent> = input
            .chatbot_intents
            .iter()
            .enumerate()
            .map(|(i, text)| ChatbotIntent {
                intent_id: format!("intent_{}", i),
                text: text.clone(),
                source_platform: "unified".to_string(),
            })
            .collect();
        let _flow_count = self.chatbot_engine.generate_flows(&intents);

        // 4) Sync platforms logically; the actual sync endpoints are outside this crate.
        let _platforms = self.chatbot_engine.sync_targets.as_list();

        // Return ALN action description for audit trails.
        AlnAction::unified_integration(
            energy_data,
            input.chatbot_intents,
            match input.blockchain {
                ChainKind::Ethereum => "Ethereum",
                ChainKind::Solana => "Solana",
                ChainKind::BitcoinL2 => "BitcoinL2",
                ChainKind::EnergyWeb => "EnergyWeb",
            },
        )
    }

    /// Helper to tokenize an energy asset as an NFT.
    pub fn tokenize_energy_asset(
        &self,
        asset: EnergyAsset,
        token_id: &str,
    ) -> EnergyAsset {
        self.energy_engine.tokenize_asset(asset, token_id)
    }

    /// Classify a gaming identifier with the ALIEN_GAMING pattern, if it matches.
    pub fn classify_gaming_id(&self, id: &str) -> Option<String> {
        let class_ = self.aln_pattern.classify(id)?;
        Some(self.aln_pattern.description(class_).to_string())
    }
}
