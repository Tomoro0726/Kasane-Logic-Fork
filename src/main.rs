use std::{collections::HashSet, fs::File};

use kasane_logic::{encode_id, encode_id_set::EncodeIDSet, space_time_id::SpaceTimeID};
use std::io::Write;
fn main() {
    let mut set1 = EncodeIDSet::new();
    let mut set2 = EncodeIDSet::new();

    let id1 = SpaceTimeID::new(5, [-1, 10], [2, 10], [5, 10], 10, [10, 40]).unwrap();
    let id2 = SpaceTimeID::new(4, [-1, 10], [2, 10], [5, 10], 10, [10, 40]).unwrap();
    let id3 = SpaceTimeID::new(1, [1, 1], [1, 1], [1, 1], 10, [10, 40]).unwrap();

    let id4 = SpaceTimeID::new(2, [2, 2], [1, 1], [1, 1], 10, [10, 40]).unwrap();
    let id5 = SpaceTimeID::new(1, [0, 0], [0, 0], [0, 0], 10, [10, 40]).unwrap();

    let mut file1 = File::create("output.txt").expect("cannot create file");

    let mut file2 = File::create("output_debug.txt").expect("cannot create file");

    println!("{},", id1);
    println!("{},", id2);
    println!("{},", id3);
    println!("{},", id4);
    println!("{},", id5);

    id1.to_encode().iter().for_each(|encode_id| {
        set1.insert(encode_id.clone());
    });

    id2.to_encode().iter().for_each(|encode_id| {
        set1.insert(encode_id.clone());
    });

    id3.to_encode().iter().for_each(|encode_id| {
        set1.insert(encode_id.clone());
    });

    id4.to_encode().iter().for_each(|encode_id| {
        set2.insert(encode_id.clone());
    });

    id5.to_encode().iter().for_each(|encode_id| {
        set2.insert(encode_id.clone());
    });

    for ele in set1.iter() {
        writeln!(file1, "{},", ele).expect("cannot write to file");
    }

    let set3 = set1.intersection(&set2);

    for ele in set3.iter() {
        writeln!(file2, "{},", ele).expect("cannot write to file");
    }
}
