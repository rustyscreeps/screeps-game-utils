use screeps::local::RoomCoordinate;

pub fn range_inclusive(
    a: RoomCoordinate,
    b: RoomCoordinate,
) -> impl DoubleEndedIterator<Item = RoomCoordinate> {
    // SAFETY: x \in [a.0, b.0], so it's in-bounds.
    (a.u8()..=b.u8()).map(|x| unsafe { RoomCoordinate::unchecked_new(x) })
}

pub fn range_exclusive(
    a: RoomCoordinate,
    b: RoomCoordinate,
) -> impl DoubleEndedIterator<Item = RoomCoordinate> {
    // SAFETY: x \in [a.0, b.0), so it's in-bounds.
    (a.u8()..b.u8()).map(|x| unsafe { RoomCoordinate::unchecked_new(x) })
}
