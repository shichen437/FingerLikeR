use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ClickParams {
    #[serde(default = "default_base_interval")]
    pub base_interval: i64,
    #[serde(default = "default_max_interval")]
    pub max_interval: i64,
    #[serde(default = "default_step")]
    pub step: i64,
    #[serde(default = "default_step_interval")]
    pub step_interval: i64,
    #[serde(default = "default_random_interval")]
    pub random_interval: i64,
    #[serde(rename = "xOffset", default = "default_offset_max")]
    pub x_offset_max: i32,
    #[serde(rename = "yOffset", default = "default_offset_max")]
    pub y_offset_max: i32,
}

fn default_base_interval() -> i64 {
    150
}
fn default_max_interval() -> i64 {
    500
}
fn default_step() -> i64 {
    200
}
fn default_step_interval() -> i64 {
    20
}
fn default_random_interval() -> i64 {
    25
}
fn default_offset_max() -> i32 {
    20
}

impl Default for ClickParams {
    fn default() -> Self {
        Self {
            base_interval: default_base_interval(),
            max_interval: default_max_interval(),
            step: default_step(),
            step_interval: default_step_interval(),
            random_interval: default_random_interval(),
            x_offset_max: default_offset_max(),
            y_offset_max: default_offset_max(),
        }
    }
}
