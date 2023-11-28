//! Functions allowing calculation of the resulting values of formulas used by
//! game mechanics related to constant values.

mod gcl;
mod gpl;
mod tower;

pub use gcl::control_points_for_gcl;
pub use gpl::power_for_gpl;
pub use tower::{
    tower_attack_power_at_range, tower_heal_power_at_range, tower_repair_power_at_range,
};
