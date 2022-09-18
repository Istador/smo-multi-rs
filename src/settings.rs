use std::{collections::{HashMap, HashSet}, net::IpAddr, sync::Arc};

use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::guid::Guid;

pub type SyncSettings = Arc<RwLock<Settings>>;
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Settings {
    pub server: ServerSettings,
    pub flip: FlipSettings,
    pub scenario: ScenarioSettings,
    pub ban_list: BanListSettings,
    pub discord: DiscordSettings,
    pub persist_shines: PersistShine,
    pub json_api: JsonApiSettings,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ServerSettings {
    pub address: IpAddr,
    pub port: u16,
    pub max_players: u16,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FlipSettings {
    pub enabled: bool,
    pub players: HashSet<Guid>,
    pub pov: FlipPovSettings,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum FlipPovSettings {
    Both,
    Player,
    Others,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ScenarioSettings {
    pub merge_enabled: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BanListSettings {
    pub enabled: bool,
    pub players: HashSet<Guid>,
    pub ips: HashSet<IpAddr>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DiscordSettings {
    pub token: Option<String>,
    pub prefix: String,
    pub log_channel: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PersistShine {
    pub enabled: bool,
    pub filename: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct JsonApiSettings {
    pub enabled: bool,
    pub tokens: HashMap<String, HashSet<String>>,
}

impl Default for ServerSettings {
    fn default() -> Self {
        Self {
            address: "0.0.0.0".parse().unwrap(),
            port: 1027,
            max_players: 8,
        }
    }
}

impl Default for FlipSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            players: Default::default(),
            pov: Default::default(),
        }
    }
}

impl Default for ScenarioSettings {
    fn default() -> Self {
        Self {
            merge_enabled: false,
        }
    }
}

impl Default for BanListSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            players: Default::default(),
            ips: Default::default(),
        }
    }
}

impl Default for DiscordSettings {
    fn default() -> Self {
        Self {
            token: Default::default(),
            prefix: "$".to_string(),
            log_channel: Default::default(),
        }
    }
}

impl Default for PersistShine {
    fn default() -> Self {
        Self {
            enabled: false,
            filename: "./moons.json".into(),
        }
    }
}

impl Default for FlipPovSettings {
    fn default() -> Self {
        Self::Both
    }
}

impl Default for JsonApiSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            tokens: Default::default(),
        }
    }
}
