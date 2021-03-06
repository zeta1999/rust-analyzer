//! Config used by the language server.
//!
//! We currently get this config from `initialize` LSP request, which is not the
//! best way to do it, but was the simplest thing we could implement.
//!
//! Of particular interest is the `feature_flags` hash map: while other fields
//! configure the server itself, feature flags are passed into analysis, and
//! tweak things like automatic insertion of `()` in completions.

use rustc_hash::FxHashMap;

use ra_project_model::CargoFeatures;
use serde::{Deserialize, Deserializer};

/// Client provided initialization options
#[derive(Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase", default)]
pub struct ServerConfig {
    /// Whether the client supports our custom highlighting publishing decorations.
    /// This is different to the highlightingOn setting, which is whether the user
    /// wants our custom highlighting to be used.
    ///
    /// Defaults to `false`
    #[serde(deserialize_with = "nullable_bool_false")]
    pub publish_decorations: bool,

    pub exclude_globs: Vec<String>,
    #[serde(deserialize_with = "nullable_bool_false")]
    pub use_client_watching: bool,

    pub lru_capacity: Option<usize>,

    #[serde(deserialize_with = "nullable_bool_true")]
    pub inlay_hints_type: bool,
    #[serde(deserialize_with = "nullable_bool_true")]
    pub inlay_hints_parameter: bool,
    pub inlay_hints_max_length: Option<usize>,

    pub cargo_watch_enable: bool,
    pub cargo_watch_args: Vec<String>,
    pub cargo_watch_command: String,
    pub cargo_watch_all_targets: bool,

    /// For internal usage to make integrated tests faster.
    #[serde(deserialize_with = "nullable_bool_true")]
    pub with_sysroot: bool,

    /// Fine grained feature flags to disable specific features.
    pub feature_flags: FxHashMap<String, bool>,

    /// Fine grained controls for additional `OUT_DIR` env variables
    pub additional_out_dirs: FxHashMap<String, String>,

    pub rustfmt_args: Vec<String>,

    /// Cargo feature configurations.
    pub cargo_features: CargoFeatures,

    /// Enabled if the vscode_lldb extension is available.
    pub vscode_lldb: bool,
}

impl Default for ServerConfig {
    fn default() -> ServerConfig {
        ServerConfig {
            publish_decorations: false,
            exclude_globs: Vec::new(),
            use_client_watching: false,
            lru_capacity: None,
            inlay_hints_type: true,
            inlay_hints_parameter: true,
            inlay_hints_max_length: None,
            cargo_watch_enable: true,
            cargo_watch_args: Vec::new(),
            cargo_watch_command: "check".to_string(),
            cargo_watch_all_targets: true,
            with_sysroot: true,
            feature_flags: FxHashMap::default(),
            additional_out_dirs: FxHashMap::default(),
            cargo_features: Default::default(),
            rustfmt_args: Vec::new(),
            vscode_lldb: false,
        }
    }
}

/// Deserializes a null value to a bool false by default
fn nullable_bool_false<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or(false))
}

/// Deserializes a null value to a bool true by default
fn nullable_bool_true<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or(true))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn deserialize_init_options_defaults() {
        // check that null == default for both fields
        let default = ServerConfig::default();
        assert_eq!(default, serde_json::from_str(r#"{}"#).unwrap());
        assert_eq!(
            default,
            serde_json::from_str(r#"{"publishDecorations":null, "lruCapacity":null}"#).unwrap()
        );
    }
}
