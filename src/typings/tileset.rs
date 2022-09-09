use crate::{CommonData, TexturePath, ViewPath};
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;

#[derive(Debug, Serialize, Deserialize, SmartDefault, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TileSet {
    #[serde(flatten)]
    pub common_data: CommonData<ConstGmTileSet>,
    pub sprite_id: Option<ViewPath>,

    pub tile_width: u64,
    pub tile_height: u64,
    pub tilexoff: u64,
    pub tileyoff: u64,
    pub tilehsep: u64,
    pub tilevsep: u64,

    pub sprite_no_export: bool,
    pub texture_group_id: TexturePath,

    #[serde(rename = "out_tilehborder")]
    pub out_tile_hborder: u64,
    #[serde(rename = "out_tilevborder")]
    pub out_tile_vborder: u64,

    #[serde(rename = "out_columns")]
    pub out_columns: u64,
    #[serde(rename = "tile_count")]
    pub tile_count: u64,
    pub auto_tile_sets: Vec<AutoTileSet>,
    pub tile_animation_frames: Vec<TileAnimationFrame>,
    pub tile_animation_speed: f64,
    pub tile_animation: TileAnimation,
    pub macro_page_tiles: MacroPageTiles,

    pub parent: crate::ViewPath,
}

#[derive(Debug, Serialize, Deserialize, SmartDefault, PartialEq, Clone)]
pub struct TileAnimation {
    #[serde(rename = "FrameData")]
    frame_data: Vec<usize>,
    #[serde(rename = "SerialiseFrameCount")]
    serialize_frame_count: usize,
}

#[derive(Debug, Serialize, Deserialize, SmartDefault, PartialEq, Clone)]
pub struct MacroPageTiles {
    #[serde(rename = "SerialiseWidth")]
    pub serialize_width: usize,
    #[serde(rename = "SerialiseHeight")]
    pub serialize_height: usize,
    #[serde(rename = "TileSerialiseData")]
    pub tile_serialize_data: Vec<usize>,
}

#[derive(Debug, Serialize, Deserialize, SmartDefault, PartialEq, Clone)]
pub struct AutoTileSet {
    #[serde(flatten)]
    common_data: CommonData<ConstGmAutoTileSet>,
    tiles: Vec<usize>,
    closed_edge: bool,
}

#[derive(Debug, Serialize, Deserialize, SmartDefault, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TileAnimationFrame {
    #[serde(flatten)]
    common_data: CommonData<ConstGmAutoTileSet>,

    pub frames: Vec<usize>,
}

#[derive(Debug, Copy, Serialize, Deserialize, SmartDefault, PartialEq, Eq, Clone)]
pub enum ConstGmTileSet {
    #[serde(rename = "GMTileSet")]
    #[default]
    Const,
}

#[derive(Debug, Copy, Serialize, Deserialize, SmartDefault, PartialEq, Eq, Clone)]
pub enum ConstGmTileAnimation {
    #[serde(rename = "GMTileAnimation")]
    #[default]
    Const,
}

#[derive(Debug, Copy, Serialize, Deserialize, SmartDefault, PartialEq, Eq, Clone)]
pub enum ConstGmAutoTileSet {
    #[serde(rename = "GMAutoTileSet")]
    #[default]
    Const,
}
