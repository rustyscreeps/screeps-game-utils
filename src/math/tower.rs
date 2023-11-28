use screeps::constants::*;

/// Provides the amount of damage done by tower attacks at a given range, after
/// accounting for reduction from [`TOWER_FALLOFF`].
///
/// [Code reference](https://github.com/screeps/engine/blob/c6c4fc9e656f160e0e0174b0dd9a817d2dd18976/src/processor/intents/towers/attack.js#L33-L38)
pub fn tower_attack_power_at_range(mut range: u8) -> u32 {
    let mut amount = TOWER_POWER_ATTACK as f64;
    if range > TOWER_FALLOFF_RANGE {
        range = TOWER_FALLOFF_RANGE;
    }
    amount -= amount * TOWER_FALLOFF * range.saturating_sub(TOWER_OPTIMAL_RANGE) as f64
        / (TOWER_FALLOFF_RANGE - TOWER_OPTIMAL_RANGE) as f64;
    amount as u32
}

/// Provides the amount of damage healed by tower healing at a given range,
/// after accounting for reduction from [`TOWER_FALLOFF`].
///
/// [Code reference](https://github.com/screeps/engine/blob/c6c4fc9e656f160e0e0174b0dd9a817d2dd18976/src/processor/intents/towers/heal.js#L24-L30)
pub fn tower_heal_power_at_range(mut range: u8) -> u32 {
    let mut amount = TOWER_POWER_HEAL as f64;
    if range > TOWER_FALLOFF_RANGE {
        range = TOWER_FALLOFF_RANGE;
    }
    amount -= amount * TOWER_FALLOFF * range.saturating_sub(TOWER_OPTIMAL_RANGE) as f64
        / (TOWER_FALLOFF_RANGE - TOWER_OPTIMAL_RANGE) as f64;
    amount as u32
}

/// Provides the amount of damage repaired by towers at a given range, after
/// accounting for reduction from [`TOWER_FALLOFF`].
///
/// [Code reference](https://github.com/screeps/engine/blob/c6c4fc9e656f160e0e0174b0dd9a817d2dd18976/src/processor/intents/towers/repair.js#L21-L26)
pub fn tower_repair_power_at_range(mut range: u8) -> u32 {
    let mut amount = TOWER_POWER_REPAIR as f64;
    if range > TOWER_FALLOFF_RANGE {
        range = TOWER_FALLOFF_RANGE;
    }
    amount -= amount * TOWER_FALLOFF * range.saturating_sub(TOWER_OPTIMAL_RANGE) as f64
        / (TOWER_FALLOFF_RANGE - TOWER_OPTIMAL_RANGE) as f64;
    amount as u32
}


#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn tower_attack_power_formula() {
        // at optimal range, the damage should be 100% of the attack power
        assert_eq!(
            tower_attack_power_at_range(TOWER_OPTIMAL_RANGE),
            TOWER_POWER_ATTACK
        );
        // at full falloff range, we should have 1 - TOWER_FALLOFF (25%) damage
        assert_eq!(
            tower_attack_power_at_range(TOWER_FALLOFF_RANGE),
            (TOWER_POWER_ATTACK as f64 * (1. - TOWER_FALLOFF)) as u32
        );
        // test values generated in js using the engine's code
        assert_eq!(tower_attack_power_at_range(5), 600);
        assert_eq!(tower_attack_power_at_range(6), 570);
        assert_eq!(tower_attack_power_at_range(7), 540);
        assert_eq!(tower_attack_power_at_range(8), 510);
        assert_eq!(tower_attack_power_at_range(9), 480);
        assert_eq!(tower_attack_power_at_range(10), 450);
        assert_eq!(tower_attack_power_at_range(11), 420);
        assert_eq!(tower_attack_power_at_range(12), 390);
        assert_eq!(tower_attack_power_at_range(13), 360);
        assert_eq!(tower_attack_power_at_range(14), 330);
        assert_eq!(tower_attack_power_at_range(15), 300);
        assert_eq!(tower_attack_power_at_range(16), 270);
        assert_eq!(tower_attack_power_at_range(17), 240);
        assert_eq!(tower_attack_power_at_range(18), 210);
        assert_eq!(tower_attack_power_at_range(19), 180);
        assert_eq!(tower_attack_power_at_range(20), 150);
        // falloff range stops at 20, make sure beyond that stays at 150
        assert_eq!(tower_attack_power_at_range(25), 150);
        // math should work even at range 0
        assert_eq!(tower_attack_power_at_range(0), 600);
    }

    #[test]
    fn tower_heal_power_formula() {
        // at optimal range, the damage should be 100% of the heal power
        assert_eq!(
            tower_heal_power_at_range(TOWER_OPTIMAL_RANGE),
            TOWER_POWER_HEAL
        );
        // at full falloff range, we should have 1 - TOWER_FALLOFF (25%) hits
        assert_eq!(
            tower_heal_power_at_range(TOWER_FALLOFF_RANGE),
            (TOWER_POWER_HEAL as f64 * (1. - TOWER_FALLOFF)) as u32
        );
        // test values generated in js using the engine's code
        assert_eq!(tower_heal_power_at_range(5), 400);
        assert_eq!(tower_heal_power_at_range(6), 380);
        assert_eq!(tower_heal_power_at_range(7), 360);
        assert_eq!(tower_heal_power_at_range(8), 340);
        assert_eq!(tower_heal_power_at_range(9), 320);
        assert_eq!(tower_heal_power_at_range(10), 300);
        assert_eq!(tower_heal_power_at_range(11), 280);
        assert_eq!(tower_heal_power_at_range(12), 260);
        assert_eq!(tower_heal_power_at_range(13), 240);
        assert_eq!(tower_heal_power_at_range(14), 220);
        assert_eq!(tower_heal_power_at_range(15), 200);
        assert_eq!(tower_heal_power_at_range(16), 180);
        assert_eq!(tower_heal_power_at_range(17), 160);
        assert_eq!(tower_heal_power_at_range(18), 140);
        assert_eq!(tower_heal_power_at_range(19), 120);
        assert_eq!(tower_heal_power_at_range(20), 100);
        // falloff range stops at 20, make sure beyond that stays at 100
        assert_eq!(tower_heal_power_at_range(25), 100);
        // math should work even at range 0
        assert_eq!(tower_heal_power_at_range(0), 400);
    }

    #[test]
    fn tower_repair_power_formula() {
        // at optimal range, the damage should be 100% of the repair power
        assert_eq!(
            tower_repair_power_at_range(TOWER_OPTIMAL_RANGE),
            TOWER_POWER_REPAIR
        );
        // at full falloff range, we should have 1 - TOWER_FALLOFF (25%) repair
        assert_eq!(
            tower_repair_power_at_range(TOWER_FALLOFF_RANGE),
            (TOWER_POWER_REPAIR as f64 * (1. - TOWER_FALLOFF)) as u32
        );
        // test values generated in js using the engine's code
        assert_eq!(tower_repair_power_at_range(5), 800);
        assert_eq!(tower_repair_power_at_range(6), 760);
        assert_eq!(tower_repair_power_at_range(7), 720);
        assert_eq!(tower_repair_power_at_range(8), 680);
        assert_eq!(tower_repair_power_at_range(9), 640);
        assert_eq!(tower_repair_power_at_range(10), 600);
        assert_eq!(tower_repair_power_at_range(11), 560);
        assert_eq!(tower_repair_power_at_range(12), 520);
        assert_eq!(tower_repair_power_at_range(13), 480);
        assert_eq!(tower_repair_power_at_range(14), 440);
        assert_eq!(tower_repair_power_at_range(15), 400);
        assert_eq!(tower_repair_power_at_range(16), 360);
        assert_eq!(tower_repair_power_at_range(17), 320);
        assert_eq!(tower_repair_power_at_range(18), 280);
        assert_eq!(tower_repair_power_at_range(19), 240);
        assert_eq!(tower_repair_power_at_range(20), 200);
        // falloff range stops at 20, make sure beyond that stays at 200
        assert_eq!(tower_repair_power_at_range(25), 200);
        // math should work even at range 0
        assert_eq!(tower_repair_power_at_range(0), 800);
    }
}
