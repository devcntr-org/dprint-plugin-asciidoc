use serde::{Serialize, Deserialize};
pub use dprint_core::configuration::{GlobalConfiguration, NewLineKind, ConfigurationDiagnostic};
pub use dprint_core::plugins::{PluginResolveConfigurationResult, FileMatchingInfo};

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Configuration {
    pub line_width: u32,
    pub indent_width: u8,
    pub use_tabs: bool,
    pub new_line_kind: NewLineKind,
}

pub fn resolve_config(config: serde_json::Value, global_config: &GlobalConfiguration) -> PluginResolveConfigurationResult<Configuration> {
    let mut diagnostics = Vec::new();
    let mut config = config;

    let resolved_config = Configuration {
        line_width: get_value(&mut config, "lineWidth", global_config.line_width.unwrap_or(80), &mut diagnostics),
        indent_width: get_value(&mut config, "indentWidth", global_config.indent_width.unwrap_or(2), &mut diagnostics),
        use_tabs: get_value(&mut config, "useTabs", global_config.use_tabs.unwrap_or(false), &mut diagnostics),
        new_line_kind: get_value(&mut config, "newLineKind", global_config.new_line_kind.clone().unwrap_or(NewLineKind::LineFeed), &mut diagnostics),
    };

    PluginResolveConfigurationResult {
        config: resolved_config,
        diagnostics,
        file_matching: FileMatchingInfo {
            file_extensions: vec!["adoc".to_string(), "asciidoc".to_string()],
            file_names: vec![],
        },
    }
}

fn get_value<T>(
    config: &mut serde_json::Value,
    key: &str,
    default_value: T,
    diagnostics: &mut Vec<ConfigurationDiagnostic>,
) -> T
where
    for<'de> T: Deserialize<'de>,
{
    if let Some(value) = config.as_object_mut().and_then(|o| o.remove(key)) {
        match serde_json::from_value::<T>(value) {
            Ok(v) => v,
            Err(err) => {
                diagnostics.push(ConfigurationDiagnostic {
                    property_name: key.to_string(),
                    message: err.to_string(),
                });
                default_value
            }
        }
    } else {
        default_value
    }
}
