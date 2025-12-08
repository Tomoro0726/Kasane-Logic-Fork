use kasane_logic::space_id::{range::RangeID, single::SingleID};

fn main() {
    let id = RangeID::new(4, [-5, 3], [3, 6], [1, 2]).unwrap();

    println!("{},", id);

    println!("{},", id.parent(1).unwrap());
    println!("{},", id.parent(2).unwrap());
}
