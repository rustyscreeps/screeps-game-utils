use screeps::constants::*;

/// Provides the total number of control points needed to achieve a given Global
/// Control Level
///
/// Calculates the total number of control points needed to achieve a given
/// Global Control Level. The game's API only exposes current level plus
/// progress toward the next level; this allows you to see how many points
/// you've spent to achieve your current level
///
/// [Code reference](https://github.com/screeps/engine/blob/6d498f2f0db4e0744fa6bf8563836d36b49b6a29/src/game/game.js#L117)
pub fn control_points_for_gcl(level: u32) -> f64 {
    ((level - 1) as f64).powf(GCL_POW) * GCL_MULTIPLY as f64
}

#[cfg(test)]
mod test {
    use assert_approx_eq::assert_approx_eq;

    use super::control_points_for_gcl;

    #[test]
    fn gcl_formula() {
        // the sanity of these values has been validated up to GCL 33
        // on the MMO game server
        assert_approx_eq!(control_points_for_gcl(1), 0.);
        assert_approx_eq!(control_points_for_gcl(2), 1000000.);
        assert_approx_eq!(control_points_for_gcl(3), 5278031.643091577);
        assert_approx_eq!(control_points_for_gcl(4), 13966610.165238237);
        assert_approx_eq!(control_points_for_gcl(5), 27857618.025475968);
        assert_approx_eq!(control_points_for_gcl(6), 47591348.46789695);
        assert_approx_eq!(control_points_for_gcl(7), 73716210.39885189);
        assert_approx_eq!(control_points_for_gcl(8), 106717414.7996562);
        assert_approx_eq!(control_points_for_gcl(9), 147033389.43962047);
        assert_approx_eq!(control_points_for_gcl(10), 195066199.50773603);
        assert_approx_eq!(control_points_for_gcl(11), 251188643.15095797);
        assert_approx_eq!(control_points_for_gcl(12), 315749334.8687436);
        assert_approx_eq!(control_points_for_gcl(13), 389076491.09393656);
        assert_approx_eq!(control_points_for_gcl(14), 471480836.66525537);
        assert_approx_eq!(control_points_for_gcl(15), 563257892.1815147);
        assert_approx_eq!(control_points_for_gcl(16), 664689811.2891247);
        assert_approx_eq!(control_points_for_gcl(17), 776046882.0533236);
        assert_approx_eq!(control_points_for_gcl(18), 897588771.9617443);
        assert_approx_eq!(control_points_for_gcl(19), 1029565573.4994452);
        assert_approx_eq!(control_points_for_gcl(20), 1172218691.9999762);
        assert_approx_eq!(control_points_for_gcl(25), 2053558031.5768352);
        assert_approx_eq!(control_points_for_gcl(30), 3234113036.1951885);
        assert_approx_eq!(control_points_for_gcl(31), 3508253856.824569);
        assert_approx_eq!(control_points_for_gcl(32), 3795491867.4194345);
        assert_approx_eq!(control_points_for_gcl(33), 4095999999.9999986);
        assert_approx_eq!(control_points_for_gcl(34), 4409947870.045006);
        assert_approx_eq!(control_points_for_gcl(35), 4737501940.897796);
        assert_approx_eq!(control_points_for_gcl(40), 6584989046.083984);
        assert_approx_eq!(control_points_for_gcl(45), 8796024362.57156);
        assert_approx_eq!(control_points_for_gcl(50), 11388606621.52188);
        assert_approx_eq!(control_points_for_gcl(100), 61592022749.941284);
        assert_approx_eq!(control_points_for_gcl(1000), 15810921110646.998);
        assert_approx_eq!(control_points_for_gcl(u32::MAX), 1.3155388150906982e29);
    }

    #[test]
    #[should_panic]
    fn bad_gcl_formula_input() {
        // players cannot be GCL 0, and subtracting 1 (as the formula does)
        // overflows the u32 - this should panic.
        control_points_for_gcl(0);
    }
}
