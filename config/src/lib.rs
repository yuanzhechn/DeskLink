use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct DeskLinkConfig {
    pub network: NetworkConfig,
    pub security: SecurityConfig,
    pub performance: PerformanceConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct NetworkConfig {
    pub target: String,
    pub bind: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct SecurityConfig {
    pub token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct PerformanceConfig {
    pub mouse_flush_ms: u64,
    pub disconnect_timeout_ms: u64,
}

impl Default for DeskLinkConfig {
    fn default() -> Self {
        Self {
            network: NetworkConfig::default(),
            security: SecurityConfig::default(),
            performance: PerformanceConfig::default(),
        }
    }
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            target: "127.0.0.1:24801".to_owned(),
            bind: "0.0.0.0:24801".to_owned(),
        }
    }
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            token: "desklink-local".to_owned(),
        }
    }
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            mouse_flush_ms: 2,
            disconnect_timeout_ms: 6_000,
        }
    }
}

impl DeskLinkConfig {
    pub fn load_optional(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        if !path.exists() {
            return Ok(Self::default());
        }
        let contents = fs::read_to_string(path)
            .with_context(|| format!("failed to read config {}", path.display()))?;
        toml::from_str(&contents)
            .with_context(|| format!("failed to parse config {}", path.display()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partial_config_uses_defaults() {
        let config: DeskLinkConfig = toml::from_str("[network]\ntarget='10.0.0.2:24801'").unwrap();
        assert_eq!(config.network.target, "10.0.0.2:24801");
        assert_eq!(config.performance.mouse_flush_ms, 2);
    }
}
