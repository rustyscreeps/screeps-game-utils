use std::collections::VecDeque;

use crate::large_cost_matrix::LargeCostMatrix;
use screeps::{
    constants::Direction,
    local::{LocalCostMatrix, LocalRoomTerrain, RoomXY},
};

/// Creates a LocalCostMatrix from LocalRoomTerrain, such that all the positions
/// which are Walls will have the value u8::MAX, and all other positions will
/// have a value of 0.
pub fn get_obstacles_lcm_from_terrain(room_terrain: &LocalRoomTerrain) -> LocalCostMatrix {
    let mut obstacles = LocalCostMatrix::new();
    for (xy, cm_val) in obstacles.iter_mut() {
        *cm_val = match room_terrain.get_xy(xy) {
            screeps::constants::Terrain::Wall => u8::MAX,
            _ => 0,
        };
    }

    obstacles
}

/// Takes a Vec of origin locations to start the floodfill from, and a Cost
/// Matrix of obstacles, and produces a `LargeCostMatrix` with distances for all
/// positions that can be reached from the origin points.
///
/// The obstacles Cost Matrix should have u8::MAX set on all positions that are
/// obstacles, and 0 everywhere else.
pub fn numerical_floodfill(
    origins: &Vec<RoomXY>,
    obstacles: &LocalCostMatrix,
    max_distance: u16,
) -> LargeCostMatrix {
    let mut output_cm = LargeCostMatrix::new_with_default(u16::MAX);

    let mut queue: VecDeque<RoomXY> = VecDeque::new();
    let mut seen = LocalCostMatrix::new();

    // Add all the valid neighbors of the origin positions to the queue to be
    // visited
    for current_position in origins {
        // The current origin position is trivially reachable from the set of origin
        // positions
        output_cm.set(*current_position, 0);

        // We've visited this origin position
        seen.set(*current_position, 1);

        let neighbor_distance = 1;

        // Check each neighbor of the current origin position for validity and add it to
        // the queue to be checked
        Direction::iter()
            .filter_map(|dir| current_position.checked_add_direction(*dir))
            .filter(|position| obstacles.get(*position) == 0)
            .for_each(|position| {
                // Only process neighbors that haven't been seen yet
                if seen.get(position) == 0 && neighbor_distance <= max_distance {
                    queue.push_back(position);
                    seen.set(position, 1);
                    output_cm.set(position, neighbor_distance);
                }
            });
    }

    // Process all entries in the queue
    let mut max_queue_length = 0;
    while !queue.is_empty() {
        let queue_length = queue.len();
        if queue_length > max_queue_length {
            max_queue_length = queue_length;
        }

        // Pop the next entry off the queue
        if let Some(current_position) = queue.pop_front() {
            // Mark current position as visited
            seen.set(current_position, 1);

            // Get the current distance value to increment for unvisited neighbors
            let current_distance = output_cm.get(current_position);
            let neighbor_distance = current_distance + 1;

            // Get list of valid neighbors and add them to the queue to be visited
            Direction::iter()
                .filter_map(|dir| current_position.checked_add_direction(*dir))
                .filter(|position| obstacles.get(*position) == 0)
                .for_each(|position| {
                    // Only process neighbors that haven't been seen yet
                    if seen.get(position) == 0 && neighbor_distance <= max_distance {
                        queue.push_back(position);
                        seen.set(position, 1);

                        // Only update the position distance if it hasn't already been set
                        if output_cm.get(position) == u16::MAX {
                            output_cm.set(position, neighbor_distance);
                        }
                    }
                });
        };
    }

    output_cm
}

/// Takes a Vec of origin locations to start the floodfill from, and a Cost
/// Matrix of obstacles, and produces a Cost Matrix with 1 values for all
/// positions that can be reached from the origin points, and 0 values
/// everywhere else.
///
/// The obstacles Cost Matrix should have u8::MAX set on all positions that are
/// obstacles, and 0 everywhere else.
pub fn reachability_floodfill(
    origins: &Vec<RoomXY>,
    obstacles: &LocalCostMatrix,
) -> LocalCostMatrix {
    let ff_lg_cm = numerical_floodfill(origins, obstacles, u16::MAX);

    let mut ret_cm = LocalCostMatrix::new();

    for (xy, cm_val) in ret_cm.iter_mut() {
        *cm_val = match ff_lg_cm.get(xy) {
            u16::MAX => 0,
            _ => 1,
        };
    }

    ret_cm
}
