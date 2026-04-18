use anyhow::Context;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct ServiceSettings {
    #[serde(default = "default_host")]
    pub host: String,
    pub port: u16,

    #[serde(default)]
    pub metrics_port: Option<u16>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct Settings {
    pub service: ServiceSettings,
}

fn default_host() -> String {
    "0.0.0.0".to_string()
}

impl Default for ServiceSettings {
    fn default() -> Self {
        Self {
            host: default_host(),
            port: 50051,
            metrics_port: Some(9090),
        }
    }
}

pub fn load_settings() -> anyhow::Result<Settings> {
    let _ = dotenvy::dotenv();

    let cfg = config::Config::builder()
        .add_source(config::Environment::default().prefix("LE").separator("__"))
        .build()
        .context("build config")?;

    let settings = cfg.try_deserialize::<Settings>().unwrap_or_default();
    Ok(settings)
}
