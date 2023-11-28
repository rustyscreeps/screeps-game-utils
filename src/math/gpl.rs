use screeps::constants::*;

/// Provides the total number of processed power needed to achieve a given
/// Global Power Level
///
/// Calculates the total number of power that need to be processed to achieve a
/// given Global Power Level. The game's API only exposes current level plus
/// progress toward the next level; this allows you to see how much you
/// processed to achieve your current level
///
/// [Code reference](https://github.com/screeps/engine/blob/6d498f2f0db4e0744fa6bf8563836d36b49b6a29/src/game/game.js#L120)
pub const fn power_for_gpl(level: u32) -> u128 {
    (level as u128).pow(POWER_LEVEL_POW) * POWER_LEVEL_MULTIPLY as u128
}

#[cfg(test)]
mod test {
    use super::power_for_gpl;

    #[test]
    fn gpl_formula() {
        // the sanity of these values has been validated up to GCL 48
        // on the MMO game server
        assert_eq!(power_for_gpl(0), 0);
        assert_eq!(power_for_gpl(1), 1_000);
        assert_eq!(power_for_gpl(2), 4_000);
        assert_eq!(power_for_gpl(3), 9_000);
        assert_eq!(power_for_gpl(4), 16_000);
        assert_eq!(power_for_gpl(5), 25_000);
        assert_eq!(power_for_gpl(6), 36_000);
        assert_eq!(power_for_gpl(7), 49_000);
        assert_eq!(power_for_gpl(8), 64_000);
        assert_eq!(power_for_gpl(9), 81_000);
        assert_eq!(power_for_gpl(10), 100_000);
        assert_eq!(power_for_gpl(50), 2_500_000);
        assert_eq!(power_for_gpl(100), 10_000_000);
        assert_eq!(power_for_gpl(1_000), 1_000_000_000);
        assert_eq!(power_for_gpl(5_000), 25_000_000_000);
        assert_eq!(power_for_gpl(10_000), 100_000_000_000);
        assert_eq!(power_for_gpl(50_000), 2_500_000_000_000);
        assert_eq!(power_for_gpl(100_000), 10_000_000_000_000);
        assert_eq!(power_for_gpl(1_000_000), 1_000_000_000_000_000);
        assert_eq!(power_for_gpl(5_000_000), 25_000_000_000_000_000);
        assert_eq!(power_for_gpl(10_000_000), 100_000_000_000_000_000);
        assert_eq!(power_for_gpl(100_000_000), 10_000_000_000_000_000_000);
        // beyond this value the return overflows a u64
        assert_eq!(power_for_gpl(135_818_791), 18_446_743_988_701_681_000);
        // must be u128 return to fit this one!
        assert_eq!(power_for_gpl(135_818_792), 18_446_744_260_339_264_000);
        assert_eq!(power_for_gpl(1_000_000_000), 1_000_000_000_000_000_000_000);
        assert_eq!(power_for_gpl(4_000_000_000), 16_000_000_000_000_000_000_000);
        assert_eq!(power_for_gpl(u32::MAX), 18_446_744_065_119_617_025_000);
    }
}
