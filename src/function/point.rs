use crate::{encode_id::EncodeID, point::Point, space_time_id::SpaceTimeID};

pub fn point<P: Point>(z: u8, point1: P) -> EncodeID {
    point1.to_id(z)
}
