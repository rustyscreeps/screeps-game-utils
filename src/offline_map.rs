use std::{collections::HashMap, fs, mem::MaybeUninit};

use screeps::{
    constants::{Density, ResourceType, ROOM_SIZE},
    game::map::RoomStatus,
    local::{LocalRoomTerrain, RawObjectId, RoomCoordinate, RoomName},
};
use serde::{
    de::{Error as _, Unexpected},
    Deserialize, Deserializer,
};

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
    pub objects: Vec<OfflineObject>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum OfflineObject {
    #[serde(rename_all = "camelCase")]
    ConstructedWall {
        #[serde(rename = "_id")]
        id: RawObjectId,
        room: RoomName,
        x: RoomCoordinate,
        y: RoomCoordinate,
    },
    #[serde(rename_all = "camelCase")]
    Controller {
        #[serde(rename = "_id")]
        id: RawObjectId,
        room: RoomName,
        x: RoomCoordinate,
        y: RoomCoordinate,

        level: u8,
    },
    #[serde(rename_all = "camelCase")]
    Extractor {
        #[serde(rename = "_id")]
        id: RawObjectId,
        room: RoomName,
        x: RoomCoordinate,
        y: RoomCoordinate,
    },
    #[serde(rename_all = "camelCase")]
    KeeperLair {
        #[serde(rename = "_id")]
        id: RawObjectId,
        room: RoomName,
        x: RoomCoordinate,
        y: RoomCoordinate,
    },
    #[serde(rename_all = "camelCase")]
    Mineral {
        #[serde(rename = "_id")]
        id: RawObjectId,
        room: RoomName,
        x: RoomCoordinate,
        y: RoomCoordinate,

        density: Density,
        mineral_type: ResourceType,
        mineral_amount: u32,
    },
    #[serde(rename_all = "camelCase")]
    Portal {
        #[serde(rename = "_id")]
        id: RawObjectId,
        room: RoomName,
        x: RoomCoordinate,
        y: RoomCoordinate,

        destination: OfflinePortalDestination,
    },
    #[serde(rename_all = "camelCase")]
    Source {
        #[serde(rename = "_id")]
        id: RawObjectId,
        room: RoomName,
        x: RoomCoordinate,
        y: RoomCoordinate,

        energy: u16,
        energy_capacity: u16,
        ticks_to_regeneration: u16,
    },
    #[serde(rename_all = "camelCase")]
    Terminal {
        #[serde(rename = "_id")]
        id: RawObjectId,
        room: RoomName,
        x: RoomCoordinate,
        y: RoomCoordinate,
    },
    #[serde(other)]
    Unknown,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum OfflinePortalDestination {
    InterRoom {
        room: RoomName,
        x: RoomCoordinate,
        y: RoomCoordinate,
    },
    InterShard {
        room: RoomName,
        shard: String,
    },
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
