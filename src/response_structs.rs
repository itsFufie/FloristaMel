use serde::{Deserialize, Serialize};

// TODO! Find a way to deserialize camelCase fields into snake_case fields.

// structs
pub struct Items {
    pub items: Vec<Item>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct Item {
    pub id: u32,
    pub name: String,
    pub data: ItemData,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct ItemData {
    pub id: String,
    pub identifiedDisplayName: String,
}
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Mobs {
    pub items: Vec<Mob>,
    pub total: u32,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Mob {
    pub id: u32,
    pub name: String,
    pub data: MobData,
}
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct MobData {
    pub id: String,
    pub name: String,
    pub level: u32,
    pub element: String,
    pub elementLevel: u8,
    pub size: String,
    pub race: String,
    pub hp: u32,
    pub baseExp: u32,
    pub jobExp: u32,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct MobInfo {
    pub id: u32,
    pub name: String,
    pub level: u32,
    pub element: String,
    pub elementLevel: u8,
    pub size: String,
    pub race: String,
    pub hp: u32,
    pub str: u32,
    pub agi: u32,
    pub vit: u32,
    pub int: u32,
    pub dex: u32,
    pub luk: u32,
    pub baseExp: u32,
    pub jobExp: u32,
    pub mvpExp: Option<u32>,
    pub def: u32,
    pub mdef: u32,
    pub attack: u32,
    pub mattack: u32,
    pub delay: u32,
    pub motion: u32,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct MobDrop {
    pub id: String,
    pub itemId: u32,
    pub chance: u32,
    pub mvpDrop: bool,
    pub item: Item,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct SpawnInfo {
    pub respawn: i32,
    pub amount: u32,
    pub map: MapInfo,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct MapInfo {
    pub id: String,
    pub data: MapData,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct MapData {
    pub displayName: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct MobWrapper {
    pub data: MobInfo,
    pub drops: Vec<MobDrop>,
    pub spawns: Vec<SpawnInfo>,
}
