use std::process::Command;

#[derive(Debug, thiserror::Error)]
pub enum DockerError {
    #[error("docker command failed: {0}")]
    CommandFailed(String),
}

/// Very thin orchestrator around the unified Docker volume pattern:
///   docker volume create unified_data
///   docker run -v unified_data:/data energy_ball
///   docker run -v unified_data:/data ai_chatbot
pub struct DockerOrchestrator {
    pub volume_name: String,
    pub energy_ball_image: String,
    pub ai_chatbot_image: String,
}

impl DockerOrchestrator {
    pub fn new(volume_name: &str, energy_ball_image: &str, ai_chatbot_image: &str) -> Self {
        Self {
            volume_name: volume_name.to_string(),
            energy_ball_image: energy_ball_image.to_string(),
            ai_chatbot_image: ai_chatbot_image.to_string(),
        }
    }

    pub fn create_volume(&self) -> Result<(), DockerError> {
        let out = Command::new("docker")
            .args(["volume", "create", &self.volume_name])
            .output()
            .map_err(|e| DockerError::CommandFailed(e.to_string()))?;
        if !out.status.success() {
            return Err(DockerError::CommandFailed(
                String::from_utf8_lossy(&out.stderr).to_string(),
            ));
        }
        Ok(())
    }

    pub fn run_energy_ball(&self) -> Result<(), DockerError> {
        let out = Command::new("docker")
            .args([
                "run",
                "-d",
                "-v",
                &format!("{}:/data", self.volume_name),
                &self.energy_ball_image,
            ])
            .output()
            .map_err(|e| DockerError::CommandFailed(e.to_string()))?;
        if !out.status.success() {
            return Err(DockerError::CommandFailed(
                String::from_utf8_lossy(&out.stderr).to_string(),
            ));
        }
        Ok(())
    }

    pub fn run_ai_chatbot(&self) -> Result<(), DockerError> {
        let out = Command::new("docker")
            .args([
                "run",
                "-d",
                "-v",
                &format!("{}:/data", self.volume_name),
                &self.ai_chatbot_image,
            ])
            .output()
            .map_err(|e| DockerError::CommandFailed(e.to_string()))?;
        if !out.status.success() {
            return Err(DockerError::CommandFailed(
                String::from_utf8_lossy(&out.stderr).to_string(),
            ));
        }
        Ok(())
    }
}
