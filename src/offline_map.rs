use std::{collections::HashMap, fs, mem::MaybeUninit};

use screeps::{
    constants::ROOM_SIZE,
    game::map::RoomStatus,
    local::{LocalRoomTerrain, RoomName},
};
use serde::{
    de::{Error as _, Unexpected},
    Deserialize, Deserializer,
};
use serde_json;

const ROOM_AREA: usize = (ROOM_SIZE as usize) * (ROOM_SIZE as usize);

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
    #[serde(deserialize_with = "deserialize_room_terrain")]
    pub terrain: LocalRoomTerrain,
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
        // "out of borders" value appears in API returns,
        // map to closed since that's effectively identical
        "out of borders" => Ok(RoomStatus::Closed),
        _ => Err(D::Error::invalid_value(
            Unexpected::Str(s),
            &"valid room status",
        )),
    }
}

fn deserialize_room_terrain<'de, D>(deserializer: D) -> Result<LocalRoomTerrain, D::Error>
where
    D: Deserializer<'de>,
{
    let s = <&'de str>::deserialize(deserializer)?;
    if s.len() == ROOM_AREA {
        let mut data: Box<[MaybeUninit<u8>; ROOM_AREA]> =
            Box::new([MaybeUninit::uninit(); ROOM_AREA]);
        for (i, c) in s.chars().enumerate() {
            let value = match c {
                '0' => 0,
                '1' => 1,
                '2' => 2,
                // leave the plain-swamps alone, against my better judgement?
                '3' => 3,
                _ => {
                    return Err(D::Error::invalid_value(
                        Unexpected::Char(c),
                        &"valid terrain integer value",
                    ))
                }
            };
            data[i].write(value);
        }
        // SAFETY: we've initialized all the bytes, because we know we had 2500 to start
        // with
        Ok(LocalRoomTerrain::new_from_bits(unsafe {
            std::mem::transmute::<Box<[MaybeUninit<u8>; ROOM_AREA]>, Box<[u8; ROOM_AREA]>>(data)
        }))
    } else {
        Err(D::Error::invalid_value(
            Unexpected::Str(s),
            &"terrain string of correct length",
        ))
    }
}

pub fn load_shard_map_json<P: AsRef<std::path::Path>>(path: P) -> OfflineShardData {
    let shard_data_json = fs::read_to_string(path).expect("readable file at specified path");
    serde_json::from_str(&shard_data_json).expect("valid shard map json")
}
