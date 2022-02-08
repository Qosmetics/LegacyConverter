use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug, Hash, Eq, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CyoobConfig {
    pub has_debris: bool,
    pub has_slider: bool,
    pub has_bomb: bool,
    pub show_arrows: bool,
    pub is_mirrorable: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_legacy: Option<bool>

}