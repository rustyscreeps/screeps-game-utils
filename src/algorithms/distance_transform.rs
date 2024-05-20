// Heavily based on https://github.com/Screeps-Tutorials/Screeps-Tutorials/blob/Master/basePlanningAlgorithms/distanceTransform.js

use screeps::{
    self,
    constants::{extra::ROOM_SIZE, Direction},
    local::{LocalCostMatrix, LocalRoomTerrain, RoomXY},
};

/// Provides a Cost Matrix with values equal to the Chebyshev distance from any
/// wall terrain. This does *not* calculate based on constructed walls, only
/// terrain walls.
pub fn chebyshev_distance_transform_from_terrain(room_terrain: &LocalRoomTerrain) -> LocalCostMatrix {
    let mut initial_cm = LocalCostMatrix::new();

    for (xy, cm_val) in initial_cm.iter_mut() {
        *cm_val = match room_terrain.get_xy(xy) {
            screeps::constants::Terrain::Wall => 0,
            _ => u8::MAX,
        };
    }
    chebyshev_distance_transform_from_cost_matrix(initial_cm)
}

/// Provides a Cost Matrix with values equal to the Chebyshev distance from any
/// position in the provided initial Cost Matrix with a value set to 0. This
/// allows for calculating the distance transform from an arbitrary set of
/// positions. Other position values in the initial Cost Matrix should be
/// initialized to 255 (u8::MAX) to ensure the calculations work correctly.
pub fn chebyshev_distance_transform_from_cost_matrix(initial_cm: LocalCostMatrix) -> LocalCostMatrix {
    // Copy the initial cost matrix into the output cost matrix
    let mut cm = initial_cm.clone();

    // Pass 1: Top-to-Bottom, Left-to-Right

    for x in 0..ROOM_SIZE {
        for y in 0..ROOM_SIZE {
            let current_position = unsafe { RoomXY::unchecked_new(x, y) };

            // The distance to the closest wall is the minimum of the current position value
            // and all of its neighbors. However, since we're going TTB:LTR, we
            // can ignore tiles we know we haven't visited yet: TopRight, Right,
            // BottomRight, and Bottom. We could include them and their default
            // max values should get ignored, but why waste the processing cycles?
            let min_value = [
                Direction::Top,
                Direction::TopLeft,
                Direction::Left,
                Direction::BottomLeft,
            ]
            .into_iter()
            .filter_map(|dir| current_position.checked_add_direction(dir))
            .map(|position| cm.get(position))
            .min()
            .map(|x| x.saturating_add(1))
            .map(|x| x.min(cm.get(current_position)))
            .unwrap_or_else(|| cm.get(current_position));

            cm.set(current_position, min_value);
        }
    }

    // Pass 2: Bottom-to-Top, Right-to-Left

    for x in (0..ROOM_SIZE).rev() {
        for y in (0..ROOM_SIZE).rev() {
            let current_position = unsafe { RoomXY::unchecked_new(x, y) };

            // The same logic as with Pass 1 applies here, we're just going BTT:RTL instead,
            // so the neighbors we ignore are: BottomLeft, Left, TopLeft, and
            // Top.
            let min_value = [
                Direction::Bottom,
                Direction::Right,
                Direction::BottomRight,
                Direction::TopRight,
            ]
            .into_iter()
            .filter_map(|dir| current_position.checked_add_direction(dir))
            .map(|position| cm.get(position))
            .min()
            .map(|x| x.saturating_add(1))
            .map(|x| x.min(cm.get(current_position)))
            .unwrap_or_else(|| cm.get(current_position));

            cm.set(current_position, min_value);
        }
    }

    cm
}
