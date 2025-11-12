use kasane_logic::{space_time_id::SpaceTimeId, space_time_id_set::SpaceTimeIdSet};

fn main() {
    let mut set = SpaceTimeIdSet::new();

    let id1 = SpaceTimeId::new(
        4,
        [Some(5), Some(15)],
        [Some(3), Some(14)],
        [Some(3), Some(12)],
        0,
        [None, None],
    )
    .unwrap();
    let id2 = SpaceTimeId::new(
        2,
        [Some(3), Some(3)],
        [Some(1), Some(3)],
        [Some(1), Some(3)],
        0,
        [None, None],
    )
    .unwrap();

    println!("{},", id1);
    println!("{}", id2);
    println!("-----------");

    set.insert(id1);
    set.insert(id2);

    for ele in set.get_all() {
        println!("{},", ele);
    }
}
