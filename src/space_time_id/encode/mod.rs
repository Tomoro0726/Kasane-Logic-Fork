use crate::{
    bit_vec::BitVec,
    encode_id::EncodeID,
    space_time_id::{
        SpaceTimeID,
        encode::{
            segment::{segment_f, segment_xy},
            to_bitvec::{to_bitvec_f, to_bitvec_xy},
        },
    },
};

pub mod segment;
pub mod to_bitvec;

use itertools::iproduct;

impl SpaceTimeID {
    pub fn to_encode(&self) -> Vec<EncodeID> {
        let f_splited = segment_f(self.z, self.f);
        let x_splited = segment_xy(self.z, self.x);
        let y_splited = segment_xy(self.z, self.y);

        let f_bitvecs: Vec<_> = f_splited.iter().map(|(z, v)| to_bitvec_f(*z, *v)).collect();

        let x_bitvecs: Vec<_> = x_splited
            .iter()
            .map(|(z, v)| to_bitvec_xy(*z, *v))
            .collect();

        let y_bitvecs: Vec<_> = y_splited
            .iter()
            .map(|(z, v)| to_bitvec_xy(*z, *v))
            .collect();

        // iproductで直積
        iproduct!(f_bitvecs, x_bitvecs, y_bitvecs)
            .map(|(f, x, y)| EncodeID { f, x, y })
            .collect()
    }
}
