Unreleased
==========

- Add `algorithms::distance_transform` module with functions for calculating distance transforms
- Add dependency on `chrono` crate for handling returned date types
- Add `object::creation_datetime` function for determining the creation timestamp of objects
  with MongoDB-style IDs

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
