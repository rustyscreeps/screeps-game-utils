use screeps::local::RoomName;

#[derive(Debug, PartialEq, Eq)]
pub enum RoomType {
    Normal,
    Highway,
    Keeper,
    Center,
}

/// Gets the `RoomType` for a given room name, assuming the map in use follows
/// the normal sector layout.
pub fn room_type_for_name(room_name: RoomName) -> RoomType {
    let x_coord = room_name.x_coord();
    let x_mod = if x_coord < 0 {
        (x_coord.abs() - 1) % 10
    } else {
        x_coord % 10
    };

    let y_coord = room_name.y_coord();
    let y_mod = if y_coord < 0 {
        (y_coord.abs() - 1) % 10
    } else {
        y_coord % 10
    };

    if x_mod == 0 || y_mod == 0 {
        RoomType::Highway
    } else if x_mod == 5 && y_mod == 5 {
        RoomType::Center
    } else if x_mod >= 4 && x_mod <= 6 && y_mod >= 4 && y_mod <= 6 {
        RoomType::Keeper
    } else {
        RoomType::Normal
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn room_types() {
        assert_eq!(
            room_type_for_name(RoomName::new("W0N0").unwrap()),
            RoomType::Highway
        );
        assert_eq!(
            room_type_for_name(RoomName::new("E0S0").unwrap()),
            RoomType::Highway
        );

        assert_eq!(
            room_type_for_name(RoomName::new("W1N1").unwrap()),
            RoomType::Normal
        );
        assert_eq!(
            room_type_for_name(RoomName::new("E1S1").unwrap()),
            RoomType::Normal
        );

        assert_eq!(
            room_type_for_name(RoomName::new("W3N3").unwrap()),
            RoomType::Normal
        );
        assert_eq!(
            room_type_for_name(RoomName::new("E3S3").unwrap()),
            RoomType::Normal
        );

        assert_eq!(
            room_type_for_name(RoomName::new("W4N4").unwrap()),
            RoomType::Keeper
        );
        assert_eq!(
            room_type_for_name(RoomName::new("E4S4").unwrap()),
            RoomType::Keeper
        );

        assert_eq!(
            room_type_for_name(RoomName::new("W5N5").unwrap()),
            RoomType::Center
        );
        assert_eq!(
            room_type_for_name(RoomName::new("E5S5").unwrap()),
            RoomType::Center
        );

        assert_eq!(
            room_type_for_name(RoomName::new("W6N6").unwrap()),
            RoomType::Keeper
        );
        assert_eq!(
            room_type_for_name(RoomName::new("E6S6").unwrap()),
            RoomType::Keeper
        );

        assert_eq!(
            room_type_for_name(RoomName::new("W7N7").unwrap()),
            RoomType::Normal
        );
        assert_eq!(
            room_type_for_name(RoomName::new("E7S7").unwrap()),
            RoomType::Normal
        );

        assert_eq!(
            room_type_for_name(RoomName::new("W9N9").unwrap()),
            RoomType::Normal
        );
        assert_eq!(
            room_type_for_name(RoomName::new("E9S9").unwrap()),
            RoomType::Normal
        );

        assert_eq!(
            room_type_for_name(RoomName::new("W10N10").unwrap()),
            RoomType::Highway
        );
        assert_eq!(
            room_type_for_name(RoomName::new("E10S10").unwrap()),
            RoomType::Highway
        );
    }
}
