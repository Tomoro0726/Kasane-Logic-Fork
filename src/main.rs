use std::collections::HashMap;

use kasane_logic::id::space_id::{
    SpaceID,
    constants::{F_MAX, F_MIN, XY_MAX},
    range::RangeID,
    single::SingleID,
};

fn main() {
    let mut id = SingleID::new(4, 6, 9, 14).unwrap();

    println!("{}", id);

    let _ = id.wrap_f(-30);

    println!("{}", id);
}
