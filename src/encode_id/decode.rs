use crate::{
    bit_vec::BitVec,
    encode_id::EncodeID,
    space_time_id::{F_MAX, SpaceTimeID},
};

impl EncodeID {
    pub fn decode(&self) -> SpaceTimeID {
        let (f_z, f_v) = to_segment_f(&self.f);
        let (x_z, x_v) = to_segment_xy(&self.x);
        let (y_z, y_v) = to_segment_xy(&self.y);
        let max_z = f_z.max(x_z).max(y_z);

        let f = if max_z == f_z {
            [f_v, f_v]
        } else {
            let k = 2_i64.pow((max_z - f_z) as u32);
            [f_v * k, (f_v + 1) * k - 1]
        };

        let x = if max_z == x_z {
            [x_v, x_v]
        } else {
            let k = 2_u64.pow((max_z - x_z) as u32);
            [x_v * k, (x_v + 1) * k - 1]
        };

        let y = if max_z == y_z {
            [y_v, y_v]
        } else {
            let k = 2_u64.pow((max_z - y_z) as u32);
            [y_v * k, (y_v + 1) * k - 1]
        };

        SpaceTimeID {
            z: max_z,
            f,
            x,
            y,
            // i: 0,
            // t: [0, u64::MAX],
        }
    }
}

pub(crate) fn to_segment_xy(bitmask: &BitVec) -> (u8, u64) {}

pub(crate) fn to_segment_f(bitmask: &BitVec) -> (u8, i64) {
    let (z, f) = to_segment_xy(bitmask);

    //仮に負の範囲の場合
    if *bitmask
        .0
        .first()
        .expect("Internal error: BitVec is empty in invert_bitmask_f")
        >= 0b11000000
    {
        return (z, -(f as i64) + F_MAX[z as usize]);
    } else {
        return (z, f as i64);
    }
}
