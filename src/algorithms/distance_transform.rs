
// Heavily based on https://github.com/Screeps-Tutorials/Screeps-Tutorials/blob/Master/basePlanningAlgorithms/distanceTransform.js

use std::cmp;

use crate::offline_map;

use screeps;
use screeps::local::{LocalCostMatrix, RoomXY};
use screeps::constants::Direction;
use screeps::constants::extra::ROOM_SIZE;


/// Provides a Cost Matrix with values equal to the Chebyshev distance from any wall terrain.
/// This does *not* calculate based on constructed walls, only terrain walls.
pub fn distance_transform(room_data: &offline_map::OfflineRoomData) -> LocalCostMatrix {
  let mut initial_cm = LocalCostMatrix::new();

  for x in 0..ROOM_SIZE {
    for y in 0..ROOM_SIZE {
      let position = unsafe { RoomXY::unchecked_new(x as u8, y as u8) };
      let val = match &room_data.terrain.get_xy(position) {
        screeps::constants::Terrain::Wall => 0,
        _ => 255,
      };

      initial_cm.set(position, val);
    }
  }

  distance_transform_from_cost_matrix(initial_cm)
}

/// Provides a Cost Matrix with values equal to the Chebyshev distance from any position
/// in the provided initial Cost Matrix with a value set to 0. This allows for calculating
/// the distance transform from an arbitrary set of positions. Other position values should
/// be set to 255 to ensure the calculations work correctly.
pub fn distance_transform_from_cost_matrix(initial_cm: LocalCostMatrix) -> LocalCostMatrix {

  // Copy the initial cost matrix into the output cost matrix
  let mut cm = initial_cm.clone();

  // Pass 1: Left-to-Right, Top-to-Bottom

  for x in 0..ROOM_SIZE {
    for y in 0..ROOM_SIZE {
      let current_position = unsafe { RoomXY::unchecked_new(x as u8, y as u8) };

      // The distance to the closest wall is the minimum of the current position value and
      // all of its neighbors. However, since we're going RTL:TTB, we can ignore tiles we
      // know we haven't visited yet: Right, BottomRight, and Bottom. We could include them
      // and their default max values should get ignored, but why waste the processing cycles?
      let mut neighbor_values: [u8; 5] = [255; 5];

      if let Some(neighbor_position) = current_position.checked_add_direction(Direction::Top) {
        let value = cm.get(neighbor_position);
        neighbor_values[0] = value;
      }

      if let Some(neighbor_position) = current_position.checked_add_direction(Direction::Left) {
        let value = cm.get(neighbor_position);
        neighbor_values[1] = value;
      }

      if let Some(neighbor_position) = current_position.checked_add_direction(Direction::TopLeft) {
        let value = cm.get(neighbor_position);
        neighbor_values[2] = value;
      }

      if let Some(neighbor_position) = current_position.checked_add_direction(Direction::TopRight) {
        let value = cm.get(neighbor_position);
        neighbor_values[3] = value;
      }

      if let Some(neighbor_position) = current_position.checked_add_direction(Direction::BottomLeft) {
        let value = cm.get(neighbor_position);
        neighbor_values[4] = value;
      }

      let current_value = cm.get(current_position);

      let neighbors_minimum = match neighbor_values.iter().min() {
        Some(value) => match value {
          255 => 255,
          _ => value + 1
        },
        None => 255,
      };

      let min_value = cmp::min(current_value, neighbors_minimum);

      cm.set(current_position, min_value);
    }
  }

  // Pass 2: Right-to-Left, Bottom-to-Top

  for x in (0..ROOM_SIZE).rev() {
    for y in (0..ROOM_SIZE).rev() {
      let current_position = unsafe { RoomXY::unchecked_new(x as u8, y as u8) };

      // The same logic as with Pass 1 applies here, we're just going RTL:BTT instead, so the
      // neighbors we ignore are: Left, TopLeft, and Top.
      let mut neighbor_values: [u8; 5] = [255; 5];

      if let Some(neighbor_position) = current_position.checked_add_direction(Direction::Bottom) {
        let value = cm.get(neighbor_position);
        neighbor_values[0] = value;
      }

      if let Some(neighbor_position) = current_position.checked_add_direction(Direction::Right) {
        let value = cm.get(neighbor_position);
        neighbor_values[1] = value;
      }

      if let Some(neighbor_position) = current_position.checked_add_direction(Direction::BottomRight) {
        let value = cm.get(neighbor_position);
        neighbor_values[2] = value;
      }

      if let Some(neighbor_position) = current_position.checked_add_direction(Direction::TopRight) {
        let value = cm.get(neighbor_position);
        neighbor_values[3] = value;
      }

      if let Some(neighbor_position) = current_position.checked_add_direction(Direction::BottomLeft) {
        let value = cm.get(neighbor_position);
        neighbor_values[4] = value;
      }

      let current_value = cm.get(current_position);

      let neighbors_minimum = match neighbor_values.iter().min() {
        Some(value) => match value {
          255 => 255,
          _ => value + 1
        },
        None => 255,
      };

      let min_value = cmp::min(current_value, neighbors_minimum);

      cm.set(current_position, min_value);
    }
  }

  return cm;
}