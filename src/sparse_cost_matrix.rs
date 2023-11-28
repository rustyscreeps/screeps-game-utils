use std::{collections::HashMap, iter::IntoIterator};

use screeps::{
    constants::ROOM_SIZE,
    local::{linear_index_to_xy, LocalCostMatrix, Position, RoomXY},
    objects::CostMatrix,
    traits::{CostMatrixGet, CostMatrixSet},
};
use serde::{Deserialize, Serialize};

pub const ROOM_AREA: usize = ROOM_SIZE as usize * ROOM_SIZE as usize;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SparseCostMatrix {
    inner: HashMap<RoomXY, u8>,
}

impl Default for SparseCostMatrix {
    fn default() -> Self {
        Self::new()
    }
}

impl SparseCostMatrix {
    pub fn new() -> Self {
        SparseCostMatrix {
            inner: HashMap::new(),
        }
    }

    pub fn get(&self, xy: RoomXY) -> u8 {
        *self.inner.get(&xy).unwrap_or(&0)
    }

    pub fn set(&mut self, xy: RoomXY, val: u8) {
        self.inner.insert(xy, val);
    }

    pub fn iter(&self) -> impl Iterator<Item = (RoomXY, u8)> + '_ {
        self.inner.iter().map(|(&pos, &val)| (pos, val))
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (RoomXY, &mut u8)> {
        self.inner.iter_mut().map(|(&pos, val)| (pos, val))
    }

    // Takes all non-zero entries in `src`, and inserts them into `self`.
    //
    // If an entry for that position exists already, overwrites it with the new
    // value.
    pub fn merge_from_dense(&mut self, src: &LocalCostMatrix) {
        self.inner.extend(src.iter().filter_map(
            |(xy, val)| {
                if val > 0 {
                    Some((xy, val))
                } else {
                    None
                }
            },
        ))
    }

    // Takes all entries in `src` and merges them into `self`.
    //
    // If an entry for that position exists already, overwrites it with the new
    // value.
    pub fn merge_from_sparse(&mut self, src: &SparseCostMatrix) {
        self.inner.extend(src.inner.iter());
    }
}

impl From<HashMap<RoomXY, u8>> for SparseCostMatrix {
    fn from(inner: HashMap<RoomXY, u8>) -> Self {
        SparseCostMatrix { inner }
    }
}

impl From<&HashMap<RoomXY, u8>> for SparseCostMatrix {
    fn from(map: &HashMap<RoomXY, u8>) -> Self {
        SparseCostMatrix { inner: map.clone() }
    }
}

impl From<&HashMap<Position, u8>> for SparseCostMatrix {
    fn from(map: &HashMap<Position, u8>) -> Self {
        SparseCostMatrix {
            inner: map.iter().map(|(&pos, &val)| (pos.into(), val)).collect(),
        }
    }
}

impl From<&CostMatrix> for SparseCostMatrix {
    fn from(js_matrix: &CostMatrix) -> Self {
        let vals: Vec<u8> = js_matrix.get_bits().to_vec();
        assert!(
            vals.len() == ROOM_AREA,
            "JS CostMatrix had length {} instead of {}.",
            vals.len(),
            ROOM_AREA
        );

        SparseCostMatrix {
            inner: vals
                .into_iter()
                .enumerate()
                .filter_map(|(idx, val)| {
                    // 0 is the same as unset, so filtering it out
                    if val > 0 {
                        Some((linear_index_to_xy(idx), val))
                    } else {
                        None
                    }
                })
                .collect(),
        }
    }
}

impl From<&LocalCostMatrix> for SparseCostMatrix {
    fn from(lcm: &LocalCostMatrix) -> Self {
        SparseCostMatrix {
            inner: lcm
                .iter()
                .filter_map(|(xy, val)| if val > 0 { Some((xy, val)) } else { None })
                .collect(),
        }
    }
}

impl From<SparseCostMatrix> for LocalCostMatrix {
    fn from(mut scm: SparseCostMatrix) -> Self {
        let mut lcm = LocalCostMatrix::new();
        for (pos, val) in scm.inner.drain() {
            lcm[pos] = val;
        }
        lcm
    }
}

impl From<&SparseCostMatrix> for LocalCostMatrix {
    fn from(scm: &SparseCostMatrix) -> Self {
        let mut lcm = LocalCostMatrix::new();
        for (&pos, &val) in scm.inner.iter() {
            lcm[pos] = val;
        }
        lcm
    }
}

impl CostMatrixSet for SparseCostMatrix {
    fn set_xy(&mut self, xy: RoomXY, cost: u8) {
        SparseCostMatrix::set(self, xy, cost);
    }
}

impl CostMatrixGet for SparseCostMatrix {
    fn get_xy(&mut self, xy: RoomXY) -> u8 {
        SparseCostMatrix::get(self, xy)
    }
}
