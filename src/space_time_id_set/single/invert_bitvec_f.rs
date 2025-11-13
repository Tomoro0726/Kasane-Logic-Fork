use crate::{bit_vec::BitVec, space_time_id_set::single::invert_bitvec_xy::invert_bitmask_xy};

pub fn invert_bitmask_f(bitmask: &BitVec) -> (u8, i64) {
    //負の範囲が考慮されていないので、おかしい

    let (z, f) = invert_bitmask_xy(bitmask);

    //+1しなければならないのはおかしい気がするが、なぜがこれで動いてしまっている
    (z, f as i64 + 1)
}
