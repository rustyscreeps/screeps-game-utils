use chrono::{DateTime, LocalResult, TimeZone, Utc};
use screeps::RawObjectId;

/// Get the date and time of an object's creation, according to its
/// MongoDB-style object ID.
///
/// Only valid when the backend server environment is using MongoDB as a
/// backend, which includes the timestamp an object is created as part of the
/// ID.
pub fn creation_datetime(object_id: RawObjectId) -> Option<DateTime<Utc>> {
    let id_packed: u128 = object_id.into();

    match Utc.timestamp_opt((id_packed >> 96) as i64, 0) {
        LocalResult::Single(dt) => Some(dt),
        _ => None,
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn id_to_datetime() {
        // Creep Worker12129506 is legendary: the oldest known creep on MMO - forever
        // renewed by Orlet, long may he *pew!*
        let id = RawObjectId::from_str("5bb64cc4f1ee994d0023982b").unwrap();
        // The start of the creep's ID, 5bb64cc4, corresponds to a timestamp of
        // Thu Oct 04 2018 17:24:20 GMT+0000
        let date = DateTime::parse_from_rfc3339("2018-10-04 17:24:20Z").unwrap();
        assert_eq!(date, creation_datetime(id).unwrap());
    }
}
