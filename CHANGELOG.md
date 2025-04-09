Unreleased
==========

0.23.0 (2025-04-09)
===================

- Updated to 0.23 of screeps-game-api crate

0.22.1 (2024-09-30)
===================

- Add iterators over various `RoomXY` and `RoomCoordinate` ranges
- Performance improvement for Chebyshev distance transform and add Manhattan distance transform

0.22.0 (2024-08-27)
===================

- Updated to 0.22 of screeps-game-api crate
- Add `algorithms::distance_transform` module with functions for calculating distance transforms
- Add dependency on `chrono` crate for handling returned date types
- Add `object::creation_datetime` function for determining the creation timestamp of objects
  with MongoDB-style IDs
- Add `map::room_type_for_name` function and `map::RoomType` enum for determining the room type
  of rooms in a default sector layout
- Add `algorithms::floodfill` module with functions for calculating flood-fills
- Add `LargeCostMatrix` struct for cost matrices that need more than `u8` sized value data

0.21.1 (2023-05-15)
===================

- Fixed update to 0.21 of screeps-game-api crate

0.21.0 (2023-05-14)
===================

- Parse objects in `OfflineRoomData` into `Vec<OfflineObject>` instead of `Vec<serde_json::Value>`
- Updated to 0.21 of screeps-game-api crate

0.20.0 (2023-01-08)
===================

- Updated to 0.20 of screeps-game-api crate

0.19.0 (2023-12-21)
===================

- Updated to 0.19 of screeps-game-api crate

0.18.0 (2023-11-27)
===================

- Add `SparseCostMatrix` (moved from screeps-game-api crate)

0.17.0 (2023-11-27)
===================

- Initial release
