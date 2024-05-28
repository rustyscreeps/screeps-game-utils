use std::ops::{Index, IndexMut};

use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use screeps::{
    constants::ROOM_SIZE,
    objects::CostMatrix,
    traits::{CostMatrixGet, CostMatrixSet},
};

use screeps::local::{linear_index_to_xy, xy_to_linear_index, LocalCostMatrix, Position, RoomXY};

pub const ROOM_AREA: usize = ROOM_SIZE as usize * ROOM_SIZE as usize;

/// A matrix of pathing costs for a room, stored in Rust memory. Stores
/// u16 values, so it can hold significantly more data in any particular
/// position compared to the default `CostMatrix`.
#[serde_as]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LargeCostMatrix {
    #[serde_as(as = "[_; ROOM_AREA]")]
    bits: [u16; ROOM_AREA],
}

impl Default for LargeCostMatrix {
    fn default() -> Self {
        Self::new()
    }
}

impl LargeCostMatrix {
    #[inline]
    pub const fn new() -> Self {
        LargeCostMatrix {
            bits: [0; ROOM_AREA],
        }
    }
    #[inline]
    pub const fn new_with_default(default: u16) -> Self {
        LargeCostMatrix {
            bits: [default; ROOM_AREA],
        }
    }

    // # Notes
    // This method does no bounds checking for the passed-in `RoomXY`, you may use
    // `RoomXY::unchecked_new` to skip all bounds checking.
    #[inline]
    pub fn set(&mut self, xy: RoomXY, val: u16) {
        self[xy] = val;
    }

    // # Notes
    // This method does no bounds checking for the passed-in `RoomXY`, you may use
    // `RoomXY::unchecked_new` to skip all bounds checking.
    #[inline]
    pub fn get(&self, xy: RoomXY) -> u16 {
        self[xy]
    }

    pub const fn get_bits(&self) -> &[u16; ROOM_AREA] {
        &self.bits
    }

    pub fn iter(&self) -> impl Iterator<Item = (RoomXY, u16)> + '_ {
        self.bits
            .iter()
            .enumerate()
            .map(|(idx, &val)| (linear_index_to_xy(idx), val))
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (RoomXY, &mut u16)> {
        self.bits
            .iter_mut()
            .enumerate()
            .map(|(idx, val)| (linear_index_to_xy(idx), val))
    }
}

impl From<LargeCostMatrix> for Vec<u16> {
    /// Returns a vector of bits length ROOM_AREA, where each position is
    /// `idx = ((x * ROOM_SIZE) + y)`.
    #[inline]
    fn from(lcm: LargeCostMatrix) -> Vec<u16> {
        lcm.bits.into()
    }
}

impl From<&LargeCostMatrix> for Vec<u16> {
    fn from(lcm: &LargeCostMatrix) -> Vec<u16> {
        lcm.bits.into()
    }
}

impl From<&CostMatrix> for LargeCostMatrix {
    fn from(js_matrix: &CostMatrix) -> Self {
        let mut bits: [u16; ROOM_AREA] = [0; ROOM_AREA];
        js_matrix
            .get_bits()
            .to_vec()
            .iter()
            .enumerate()
            .for_each(|(idx, &val)| bits[idx] = val.into());

        LargeCostMatrix { bits }
    }
}

impl Index<RoomXY> for LargeCostMatrix {
    type Output = u16;

    fn index(&self, xy: RoomXY) -> &Self::Output {
        // SAFETY: RoomXY is always a valid coordinate.
        unsafe { self.bits.get_unchecked(xy_to_linear_index(xy)) }
    }
}

impl IndexMut<RoomXY> for LargeCostMatrix {
    fn index_mut(&mut self, xy: RoomXY) -> &mut Self::Output {
        // SAFETY: RoomXY is always a valid coordinate.
        unsafe { self.bits.get_unchecked_mut(xy_to_linear_index(xy)) }
    }
}

impl Index<Position> for LargeCostMatrix {
    type Output = u16;

    fn index(&self, idx: Position) -> &Self::Output {
        &self[RoomXY::from(idx)]
    }
}

impl IndexMut<Position> for LargeCostMatrix {
    fn index_mut(&mut self, idx: Position) -> &mut Self::Output {
        &mut self[RoomXY::from(idx)]
    }
}

impl CostMatrixSet for LargeCostMatrix {
    fn set_xy(&mut self, xy: RoomXY, cost: u8) {
        LargeCostMatrix::set(self, xy, cost as u16);
    }
}

impl CostMatrixGet for LargeCostMatrix {
    fn get_xy(&mut self, xy: RoomXY) -> u8 {
        match u8::try_from(LargeCostMatrix::get(self, xy)) {
            Ok(var) => var,
            Err(_) => u8::MAX,
        }
    }
}

impl From<LargeCostMatrix> for LocalCostMatrix {
    fn from(lcm: LargeCostMatrix) -> LocalCostMatrix {
        let mut ret_lcm = LocalCostMatrix::new();

        lcm.bits
            .to_vec()
            .iter()
            .enumerate()
            .for_each(|(idx, &val)| {
                ret_lcm.set(linear_index_to_xy(idx), val.try_into().unwrap_or(u8::MAX))
            });

        ret_lcm
    }
}

impl From<&LargeCostMatrix> for LocalCostMatrix {
    fn from(lcm: &LargeCostMatrix) -> LocalCostMatrix {
        let mut ret_lcm = LocalCostMatrix::new();

        lcm.bits
            .to_vec()
            .iter()
            .enumerate()
            .for_each(|(idx, &val)| {
                ret_lcm.set(linear_index_to_xy(idx), val.try_into().unwrap_or(u8::MAX))
            });

        ret_lcm
    }
}

impl From<LargeCostMatrix> for CostMatrix {
    fn from(lcm: LargeCostMatrix) -> CostMatrix {
        let mut bits: [u8; ROOM_AREA] = [0; ROOM_AREA];
        lcm.bits
            .to_vec()
            .iter()
            .enumerate()
            .for_each(|(idx, &val)| bits[idx] = val.try_into().unwrap_or(u8::MAX));

        CostMatrix::new_from_bits(&bits)
    }
}
