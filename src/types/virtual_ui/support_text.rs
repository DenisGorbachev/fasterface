use derive_more::From;
use SupportText::*;

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
pub enum SupportText {
    Developers { token: String },
    Development { token: String },
    ThisPluginDevelopment { token: String },
    ThisPlugin { token: String },
    Plugin { token: String, plugin: String },
}

impl SupportText {}

impl From<SupportText> for String {
    fn from(value: SupportText) -> Self {
        match value {
            Developers {
                token,
            } => format!("Buy {token} to support developers"),
            Development {
                token,
            } => format!("Buy {token} to support development"),
            ThisPluginDevelopment {
                token,
            } => format!("Buy {token} to support this plugin development"),
            ThisPlugin {
                token,
            } => format!("Buy {token} to support this plugin"),
            Plugin {
                token,
                plugin,
            } => format!("Buy {token} to support {plugin}"),
        }
    }
}

/// Dummy implementation
impl Default for SupportText {
    fn default() -> Self {
        Development {
            token: "COLI".to_string(),
        }
    }
}
