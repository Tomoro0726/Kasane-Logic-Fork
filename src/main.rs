use std::collections::HashMap;

use kasane_logic::id::space_id::{SpaceID, range::RangeID, single::SingleID};

fn main() {
    let mut id = RangeID::new(4, [-5, 3], [3, 6], [1, 2]).unwrap();

    println!("{},", id);

    id.wrap_up(3);

    println!("{},", id);
}
