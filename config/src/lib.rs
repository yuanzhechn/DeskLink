use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct DeskLinkConfig {
    pub network: NetworkConfig,
    pub security: SecurityConfig,
    pub performance: PerformanceConfig,
    pub topology: TopologyConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct NetworkConfig {
    pub target: String,
    pub bind: String,
    pub ui_bind: String,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct TopologyConfig {
    pub enabled: bool,
    pub edge: String,
    pub remote_width: u32,
    pub remote_height: u32,
    pub remote_x: Option<i32>,
    pub remote_y: Option<i32>,
    pub enter_margin_px: i32,
    pub edge_delay_ms: u64,
    pub return_cooldown_ms: u64,
    pub windows_layout_signature: Option<String>,
}

impl Default for DeskLinkConfig {
    fn default() -> Self {
        Self {
            network: NetworkConfig::default(),
            security: SecurityConfig::default(),
            performance: PerformanceConfig::default(),
            topology: TopologyConfig::default(),
        }
    }
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            target: "127.0.0.1:24801".to_owned(),
            bind: "0.0.0.0:24801".to_owned(),
            ui_bind: "127.0.0.1:24802".to_owned(),
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

impl Default for TopologyConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            edge: "right".to_owned(),
            remote_width: 1920,
            remote_height: 1080,
            remote_x: None,
            remote_y: None,
            enter_margin_px: 1,
            edge_delay_ms: 80,
            return_cooldown_ms: 500,
            windows_layout_signature: None,
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

    pub fn save(&self, path: impl AsRef<Path>) -> Result<()> {
        let path = path.as_ref();
        let contents = toml::to_string_pretty(self).context("failed to serialize config")?;
        fs::write(path, contents)
            .with_context(|| format!("failed to save config {}", path.display()))
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
