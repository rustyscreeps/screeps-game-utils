use screeps::local::RoomCoordinate;

/// Iterate over the range of [`RoomCoordinate`]s from `a` to `b`, including both endpoints.
///
/// Safe to call even when `a > b`, will just yield an empty range.
///
/// # Example
///
/// ```
/// use screeps::local::RoomCoordinate;
/// use screeps_utils::room_coordinate::range_inclusive;
///
/// let coords: Vec<u8> = range_inclusive(RoomCoordinate::new(0).unwrap(), RoomCoordinate::new(10).unwrap())
///     .map(|coord| coord.u8())
///     .collect();
/// assert_eq!(coords, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10_u8]);
/// ```
pub fn range_inclusive(
    a: RoomCoordinate,
    b: RoomCoordinate,
) -> impl DoubleEndedIterator<Item = RoomCoordinate> {
    // SAFETY: x \in [a.0, b.0], so it's in-bounds.
    (a.u8()..=b.u8()).map(|x| unsafe { RoomCoordinate::unchecked_new(x) })
}

/// Iterate over the range of [`RoomCoordinates`]s from `a` to `b`, excluding both endpoints.
///
/// Safe to call even when `a >= b`, will just yield an empty range.
///
/// # Example
///
/// ```
/// use screeps::local::RoomCoordinate;
/// use screeps_utils::room_coordinate::range_exclusive;
///
/// let coords: Vec<u8> = range_exclusive(RoomCoordinate::new(0).unwrap(), RoomCoordinate::new(10).unwrap())
///     .map(|coord| coord.u8())
///     .collect();
/// assert_eq!(coords, [1, 2, 3, 4, 5, 6, 7, 8, 9_u8]);
///
/// // Works for empty ranges too.
/// let coords: Vec<u8> = range_exclusive(RoomCoordinate::new(0).unwrap(), RoomCoordinate::new(1).unwrap())
///     .map(|coord| coord.u8())
///     .collect();
/// assert!(coords.is_empty());
/// ```
pub fn range_exclusive(
    a: RoomCoordinate,
    b: RoomCoordinate,
) -> impl DoubleEndedIterator<Item = RoomCoordinate> {
    // SAFETY: x \in (a.0, b.0), so it's in-bounds.
    (a.u8() + 1..b.u8()).map(|x| unsafe { RoomCoordinate::unchecked_new(x) })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_inclusive_reverse() {
        let coords: Vec<u8> = range_inclusive(
            RoomCoordinate::new(10).unwrap(),
            RoomCoordinate::new(20).unwrap(),
        )
        .map(|coord| coord.u8())
        .rev()
        .collect();
        assert_eq!(coords, [20_u8, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10]);
    }

    #[test]
    fn test_inclusive_a_geq_b() {
        let coords: Vec<_> = range_inclusive(
            RoomCoordinate::new(30).unwrap(),
            RoomCoordinate::new(30).unwrap(),
        )
        .collect();
        assert_eq!(coords, [RoomCoordinate::new(30).unwrap()]);

        let coords: Vec<_> = range_inclusive(
            RoomCoordinate::new(1).unwrap(),
            RoomCoordinate::new(0).unwrap(),
        )
        .collect();
        assert!(coords.is_empty());
    }

    #[test]
    fn test_exclusive_reverse() {
        let coords: Vec<u8> = range_exclusive(
            RoomCoordinate::new(10).unwrap(),
            RoomCoordinate::new(20).unwrap(),
        )
        .map(|coord| coord.u8())
        .rev()
        .collect();
        assert_eq!(coords, [19_u8, 18, 17, 16, 15, 14, 13, 12, 11]);
    }

    #[test]
    fn test_exclusive_a_geq_b() {
        let coords: Vec<_> = range_exclusive(
            RoomCoordinate::new(49).unwrap(),
            RoomCoordinate::new(49).unwrap(),
        )
        .collect();
        assert!(coords.is_empty());

        let coords: Vec<_> = range_exclusive(
            RoomCoordinate::new(49).unwrap(),
            RoomCoordinate::new(48).unwrap(),
        )
        .collect();
        assert!(coords.is_empty());
    }
}
