pub mod configuration;
pub mod format;

#[cfg(target_arch = "wasm32")]
use configuration::{Configuration, resolve_config};
#[cfg(target_arch = "wasm32")]
use dprint_core::configuration::ConfigKeyMap;
#[cfg(target_arch = "wasm32")]
use dprint_core::plugins::{PluginInfo, SyncPluginHandler, SyncFormatRequest, SyncHostFormatRequest, FormatResult, PluginResolveConfigurationResult, CheckConfigUpdatesMessage, ConfigChange};
#[cfg(target_arch = "wasm32")]
use dprint_core::generate_plugin_code;

#[cfg(target_arch = "wasm32")]
struct AsciiDocPluginHandler;

#[cfg(target_arch = "wasm32")]
impl SyncPluginHandler<Configuration> for AsciiDocPluginHandler {
    fn resolve_config(&mut self, config: ConfigKeyMap, global_config: &dprint_core::configuration::GlobalConfiguration) -> PluginResolveConfigurationResult<Configuration> {
        let config_value = serde_json::to_value(config).unwrap_or(serde_json::Value::Object(serde_json::Map::new()));
        resolve_config(config_value, global_config)
    }

    fn plugin_info(&mut self) -> PluginInfo {
        PluginInfo {
            name: env!("CARGO_PKG_NAME").to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            config_key: "asciidoc".to_string(),
            help_url: "https://dprint.dev/plugins/asciidoc".to_string(),
            config_schema_url: "".to_string(),
            update_url: None,
        }
    }

    fn license_text(&mut self) -> String {
        include_str!("../LICENSE").to_string()
    }

    fn format(
        &mut self,
        request: SyncFormatRequest<'_, Configuration>,
        _format_with_host: impl FnMut(SyncHostFormatRequest) -> FormatResult,
    ) -> anyhow::Result<Option<Vec<u8>>> {
        let file_text = String::from_utf8(request.file_bytes)?;
        match format::format_text(&file_text, request.config)? {
            Some(formatted_text) => Ok(Some(formatted_text.into_bytes())),
            None => Ok(None),
        }
    }

    fn check_config_updates(&self, _message: CheckConfigUpdatesMessage) -> anyhow::Result<Vec<ConfigChange>> {
        Ok(Vec::new())
    }
}

#[cfg(target_arch = "wasm32")]
impl AsciiDocPluginHandler {
    pub const fn new() -> Self {
        AsciiDocPluginHandler
    }
}

#[cfg(target_arch = "wasm32")]
generate_plugin_code!(AsciiDocPluginHandler, AsciiDocPluginHandler::new());
