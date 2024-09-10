// Heavily based on https://github.com/Screeps-Tutorials/Screeps-Tutorials/blob/Master/basePlanningAlgorithms/distanceTransform.js

use screeps::{
    constants::extra::ROOM_SIZE,
    local::{LocalCostMatrix, LocalRoomTerrain, RoomCoordinate, RoomXY},
};

use crate::room_coordinate::{range_exclusive, range_inclusive};

/// Provides a Cost Matrix with values equal to the Chebyshev distance from any
/// wall terrain. This does *not* calculate based on constructed walls, only
/// terrain walls.
pub fn chebyshev_distance_transform_from_terrain(
    room_terrain: &LocalRoomTerrain,
) -> LocalCostMatrix {
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
/// position in the provided initial Cost Matrix with a value set to 0.
///
/// This allows for calculating the distance transform from an arbitrary set of
/// positions. Other position values in the initial Cost Matrix should be
/// initialized to 255 (u8::MAX) to ensure the calculations work correctly.
pub fn chebyshev_distance_transform_from_cost_matrix(mut cm: LocalCostMatrix) -> LocalCostMatrix {
    let zero = RoomCoordinate::new(0).unwrap();
    let one = RoomCoordinate::new(1).unwrap();
    let forty_eight = RoomCoordinate::new(ROOM_SIZE - 2).unwrap();
    let forty_nine = RoomCoordinate::new(ROOM_SIZE - 1).unwrap();
    // Pass 1: Top-to-Bottom, Left-to-Right

    // Phase A: first column
    range_inclusive(one, forty_nine)
        .map(|y| RoomXY { x: zero, y })
        .fold(cm.get(RoomXY { x: zero, y: zero }), |top, xy| {
            let val = cm.get(xy).min(top.saturating_add(1));
            cm.set(xy, val);
            val
        });

    // Phase B: the rest
    range_inclusive(one, forty_nine)
        .zip(range_inclusive(zero, forty_eight))
        .for_each(|(current_x, left_x)| {
            let initial_top = cm
                .get(RoomXY {
                    x: current_x,
                    y: zero,
                })
                .min(
                    cm.get(RoomXY { x: left_x, y: zero })
                        .min(cm.get(RoomXY { x: left_x, y: one }))
                        .saturating_add(1),
                );
            cm.set(
                RoomXY {
                    x: current_x,
                    y: zero,
                },
                initial_top,
            );
            let final_top = range_exclusive(zero, forty_nine)
                .map(|y| {
                    (RoomXY { x: current_x, y }, unsafe {
                        [
                            RoomCoordinate::unchecked_new(y.u8() - 1),
                            y,
                            RoomCoordinate::unchecked_new(y.u8() + 1),
                        ]
                    })
                })
                .fold(initial_top, |top, (current_xy, lefts)| {
                    let val = lefts
                        .into_iter()
                        .map(|y| RoomXY { x: left_x, y })
                        .map(|xy| cm.get(xy))
                        .min()
                        .unwrap()
                        .min(top)
                        .saturating_add(1)
                        .min(cm.get(current_xy));
                    cm.set(current_xy, val);
                    val
                });
            cm.set(
                RoomXY {
                    x: current_x,
                    y: forty_nine,
                },
                cm.get(RoomXY {
                    x: current_x,
                    y: forty_nine,
                })
                .min(
                    final_top
                        .min(cm.get(RoomXY {
                            x: left_x,
                            y: forty_eight,
                        }))
                        .min(cm.get(RoomXY {
                            x: left_x,
                            y: forty_nine,
                        }))
                        .saturating_add(1),
                ),
            );
        });

    // Pass 2: Bottom-to-Top, Right-to-Left

    // Phase A: last column
    range_inclusive(zero, forty_eight)
        .map(|y| RoomXY { x: forty_nine, y })
        .rfold(
            cm.get(RoomXY {
                x: forty_nine,
                y: forty_nine,
            }),
            |bottom, xy| {
                let val = cm.get(xy).min(bottom.saturating_add(1));
                cm.set(xy, val);
                val
            },
        );

    // Phase B: the rest
    range_inclusive(zero, forty_eight)
        .rev()
        .zip(range_inclusive(one, forty_nine).rev())
        .for_each(|(current_x, right_x)| {
            let initial_bottom = cm
                .get(RoomXY {
                    x: current_x,
                    y: forty_nine,
                })
                .min(
                    cm.get(RoomXY {
                        x: right_x,
                        y: forty_nine,
                    })
                    .min(cm.get(RoomXY {
                        x: right_x,
                        y: forty_eight,
                    }))
                    .saturating_add(1),
                );
            cm.set(
                RoomXY {
                    x: current_x,
                    y: forty_nine,
                },
                initial_bottom,
            );
            let final_bottom = range_exclusive(zero, forty_nine)
                .map(|y| {
                    (RoomXY { x: current_x, y }, unsafe {
                        [
                            RoomCoordinate::unchecked_new(y.u8() - 1),
                            y,
                            RoomCoordinate::unchecked_new(y.u8() + 1),
                        ]
                    })
                })
                .rfold(initial_bottom, |bottom, (current_xy, rights)| {
                    let val = rights
                        .into_iter()
                        .map(|y| RoomXY { x: right_x, y })
                        .map(|xy| cm.get(xy))
                        .min()
                        .unwrap()
                        .min(bottom)
                        .saturating_add(1)
                        .min(cm.get(current_xy));
                    cm.set(current_xy, val);
                    val
                });
            cm.set(
                RoomXY {
                    x: current_x,
                    y: zero,
                },
                cm.get(RoomXY {
                    x: current_x,
                    y: zero,
                })
                .min(
                    final_bottom
                        .min(cm.get(RoomXY {
                            x: right_x,
                            y: zero,
                        }))
                        .min(cm.get(RoomXY { x: right_x, y: one }))
                        .saturating_add(1),
                ),
            );
        });

    cm
}

/// Provides a Cost Matrix with values equal to the Manhattan distance from any
/// wall terrain. This does *not* calculate based on constructed walls, only
/// terrain walls.
pub fn manhattan_distance_transform_from_terrain(
    room_terrain: &LocalRoomTerrain,
) -> LocalCostMatrix {
    let mut initial_cm = LocalCostMatrix::new();

    for (xy, cm_val) in initial_cm.iter_mut() {
        *cm_val = match room_terrain.get_xy(xy) {
            screeps::constants::Terrain::Wall => 0,
            _ => u8::MAX,
        };
    }
    manhattan_distance_transform_from_cost_matrix(initial_cm)
}

/// Provides a Cost Matrix with values equal to the Manhattan distance from any
/// position in the provided initial Cost Matrix with a value set to 0.
///
/// This allows for calculating the distance transform from an arbitrary set of
/// positions. Other position values in the initial Cost Matrix should be
/// initialized to 255 (u8::MAX) to ensure the calculations work correctly.
pub fn manhattan_distance_transform_from_cost_matrix(mut cm: LocalCostMatrix) -> LocalCostMatrix {
    let zero = RoomCoordinate::new(0).unwrap();
    let one = RoomCoordinate::new(1).unwrap();
    let forty_eight = RoomCoordinate::new(ROOM_SIZE - 2).unwrap();
    let forty_nine = RoomCoordinate::new(ROOM_SIZE - 1).unwrap();
    // Pass 1: Top-to-Bottom, Left-to-Right

    // Phase A: first column
    range_inclusive(one, forty_nine)
        .map(|y| RoomXY { x: zero, y })
        .fold(cm.get(RoomXY { x: zero, y: zero }), |top, xy| {
            let val = cm.get(xy).min(top.saturating_add(1));
            cm.set(xy, val);
            val
        });

    // Phase B: the rest
    range_inclusive(one, forty_nine)
        .zip(range_inclusive(zero, forty_eight))
        .for_each(|(current_x, left_x)| {
            let initial_top = cm
                .get(RoomXY {
                    x: current_x,
                    y: zero,
                })
                .min(cm.get(RoomXY { x: left_x, y: zero }).saturating_add(1));
            cm.set(
                RoomXY {
                    x: current_x,
                    y: zero,
                },
                initial_top,
            );
            range_inclusive(one, forty_nine)
                .map(|y| (RoomXY { x: current_x, y }, RoomXY { x: left_x, y }))
                .fold(initial_top, |top, (current_xy, left_xy)| {
                    let val = cm
                        .get(left_xy)
                        .min(top)
                        .saturating_add(1)
                        .min(cm.get(current_xy));
                    cm.set(current_xy, val);
                    val
                });
        });

    // Pass 2: Bottom-to-Top, Right-to-Left

    // Phase A: last column
    range_inclusive(zero, forty_eight)
        .map(|y| RoomXY { x: forty_nine, y })
        .rfold(
            cm.get(RoomXY {
                x: forty_nine,
                y: forty_nine,
            }),
            |bottom, xy| {
                let val = cm.get(xy).min(bottom.saturating_add(1));
                cm.set(xy, val);
                val
            },
        );

    // Phase B: the rest
    range_inclusive(zero, forty_eight)
        .rev()
        .zip(range_inclusive(one, forty_nine).rev())
        .for_each(|(current_x, right_x)| {
            let initial_bottom = cm
                .get(RoomXY {
                    x: current_x,
                    y: forty_nine,
                })
                .min(
                    cm.get(RoomXY {
                        x: right_x,
                        y: forty_nine,
                    })
                    .saturating_add(1),
                );
            cm.set(
                RoomXY {
                    x: current_x,
                    y: forty_nine,
                },
                initial_bottom,
            );
            range_inclusive(zero, forty_eight)
                .map(|y| (RoomXY { x: current_x, y }, RoomXY { x: right_x, y }))
                .rfold(initial_bottom, |bottom, (current_xy, right_xy)| {
                    let val = cm
                        .get(right_xy)
                        .min(bottom)
                        .saturating_add(1)
                        .min(cm.get(current_xy));
                    cm.set(current_xy, val);
                    val
                });
        });

    cm
}
