use std::collections::HashSet;

use kasane_logic::{
    space_time_id::SpaceTimeId,
    space_time_id_set::single::{
        convert_bitvec_f::convert_bitmask_f, convert_bitvec_xy::convert_bitmask_xy,
        convert_single_f::convert_f, convert_single_xy::convert_xy,
        invert_bitvec_f::invert_bitmask_f, invert_bitvec_xy::invert_bitmask_xy,
    },
    r#type::bit_vec::BitVec,
};

fn main() {
    let id = SpaceTimeId::random_z_max(10);

    let f_splited = convert_f(id.z, id.f);
    let x_splited = convert_xy(id.z, id.x);
    let y_splited = convert_xy(id.z, id.y);

    let f_encoded: Vec<BitVec> = f_splited
        .iter()
        .map(|(z, f)| convert_bitmask_f(*z, *f))
        .collect();
    let x_encoded: Vec<BitVec> = x_splited
        .iter()
        .map(|(z, x)| convert_bitmask_xy(*z, *x))
        .collect();
    let y_encoded: Vec<BitVec> = y_splited
        .iter()
        .map(|(z, y)| convert_bitmask_xy(*z, *y))
        .collect();

    println!("{}", id);

    for f_bit in f_encoded {
        println!("-----");

        println!("Start:{}", f_bit);
        println!("End  :{}", f_bit.under_prefix());

        if f_bit == f_bit.under_prefix() {
            println!("変化なし");
        }
    }
    for x_bit in x_encoded {
        println!("-----");
        println!("Start:{}", x_bit);
        println!("End  :{}", x_bit.under_prefix());

        if x_bit == x_bit.under_prefix() {
            println!("変化なし");
        }
    }
    for y_bit in y_encoded {
        println!("-----");
        println!("Start:{}", y_bit);
        println!("End  :{}", y_bit.under_prefix());

        if y_bit == y_bit.under_prefix() {
            println!("変化なし");
        }
    }
}
