use screeps::{game::map::RoomStatus, local::RoomName};
use serde::{
    de::{Error as _, Unexpected},
    Deserialize, Deserializer,
};
use serde_json;
use std::{collections::HashMap, fs};

#[derive(Deserialize, Debug)]
pub struct OfflineShardData {
    /// A text description of the map dump
    pub description: String,
    /// Each room's entry in the map dump
    #[serde(deserialize_with = "deserialize_offline_rooms")]
    pub rooms: HashMap<RoomName, OfflineRoomData>,
}

#[derive(Deserialize, Debug)]
pub struct OfflineRoomData {
    #[serde(rename = "room")]
    pub room_name: RoomName,
    #[serde(deserialize_with = "deserialize_room_status")]
    pub status: RoomStatus,
    /// Whether the room is a highway room
    #[serde(default)]
    pub bus: bool,
    // todo get this converted into something usable -
    // I guess the local terrain type
    pub terrain: String,
    // todo need object wrappers
    pub objects: Vec<serde_json::Value>,
}

fn deserialize_offline_rooms<'de, D>(
    deserializer: D,
) -> Result<HashMap<RoomName, OfflineRoomData>, D::Error>
where
    D: Deserializer<'de>,
{
    let mut rooms = HashMap::new();
    for room in Vec::<OfflineRoomData>::deserialize(deserializer)? {
        rooms.insert(room.room_name, room);
    }
    Ok(rooms)
}

fn deserialize_room_status<'de, D>(deserializer: D) -> Result<RoomStatus, D::Error>
where
    D: Deserializer<'de>,
{
    let s = <&'de str>::deserialize(deserializer)?;
    match s {
        "normal" => Ok(RoomStatus::Normal),
        "closed" => Ok(RoomStatus::Closed),
        "novice" => Ok(RoomStatus::Novice),
        "respawn" => Ok(RoomStatus::Respawn),
        // this value appears in exported rooms from maps.screepspl.us,
        // map to closed since that's effectively identical
        "out of borders" => Ok(RoomStatus::Closed),
        _ => Err(D::Error::invalid_value(
            Unexpected::Str(s),
            &"valid room status",
        )),
    }
}

pub fn load_shard_map_json<P: AsRef<std::path::Path>>(path: P) -> OfflineShardData {
    let shard_data_json = fs::read_to_string(path).expect("readable file at specified path");
    serde_json::from_str(&shard_data_json).expect("valid shard map json")
}
