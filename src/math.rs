//! Functions allowing calculation of the resulting values of formulas used by
//! game mechanics related to constant values.

mod gcl;
mod gpl;

pub use gcl::control_points_for_gcl;
pub use gpl::power_for_gpl;
