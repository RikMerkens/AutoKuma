use serde::{Deserialize, Serialize};
use serde_alias::serde_alias;
use serde_inline_default::serde_inline_default;

#[serde_alias(ScreamingSnakeCase)]
#[serde_inline_default]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DockerConfig {
    /// Wether docker integration should be enabled or not.
    #[serde_inline_default(true)]
    pub enabled: bool,

    /// Path to the Docker socket.
    #[serde_inline_default("/var/run/docker.sock".to_owned())]
    pub socket_path: String,

    /// Prefix used when scanning for container labels.
    #[serde_inline_default("kuma".to_owned())]
    pub label_prefix: String,
}

#[serde_alias(ScreamingSnakeCase)]
#[serde_inline_default]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub kuma: kuma_client::Config,

    pub docker: DockerConfig,

    /// The interval in between syncs.
    #[serde_inline_default(5.0)]
    pub sync_interval: f64,

    /// The path to the folder in which AutoKuma will search for static Monitor definitions.
    #[serde_inline_default("/monitors".to_owned())]
    pub static_monitors: String,

    /// The name of the AutoKuma tag, used to track managed containers
    #[serde_inline_default("AutoKuma".to_owned())]
    pub tag_name: String,

    /// The color of the AutoKuma tag
    #[serde_inline_default("#42C0FB".to_owned())]
    pub tag_color: String,

    /// Default settings applied to all generated Monitors.
    #[serde_inline_default("".to_owned())]
    pub default_settings: String,
}
