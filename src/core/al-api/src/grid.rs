use serde::{Deserialize, Serialize};

use crate::angle::Formatter;

use super::color::ColorRGB;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GridCfg {
    #[serde(default = "default_color")]
    pub color: Option<ColorRGB>,
    #[serde(default = "default_thickness")]
    pub thickness: Option<f32>,
    pub opacity: Option<f32>,
    #[serde(default = "default_labels")]
    pub show_labels: Option<bool>,
    #[serde(default = "default_label_size")]
    pub label_size: Option<f32>,
    #[serde(default = "default_enabled")]
    pub enabled: Option<bool>,
    #[serde(default = "default_fmt")]
    pub fmt: Option<Formatter>,
}

fn default_labels() -> Option<bool> {
    None
}

fn default_enabled() -> Option<bool> {
    None
}

fn default_color() -> Option<ColorRGB> {
    None
}

fn default_label_size() -> Option<f32> {
    None
}

fn default_thickness() -> Option<f32> {
    None
}

fn default_fmt() -> Option<Formatter> {
    None
}
