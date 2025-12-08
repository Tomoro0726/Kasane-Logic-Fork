use crate::{
    bit_vec::BitVec,
    space_id::{constants::F_MAX, segment::Segment},
};

impl BitVec {
    pub fn to_segment_f(&self) -> Segment<i64> {
        let segment = self.to_segment_xy();

        if *self
            .0
            .first()
            .expect("Internal error: BitVec is empty in invert_bitmask_f")
            >= 0b11000000
        {
            return Segment {
                z: segment.z,
                dim: -(segment.dim as i64) + F_MAX[segment.z as usize],
            };
        } else {
            return Segment {
                z: segment.z,
                dim: segment.dim as i64,
            };
        }
    }

    pub fn to_segment_xy(&self) -> Segment<u64> {
        let bytes = &self.0;
        let total_bits = bytes.len() * 8;
        let total_layers = (total_bits + 1) / 2;

        let mut uxy: u64 = 0;
        let mut max_z: i32 = -1; // 見つかった最大のz

        // now_z=0 から順に処理
        for now_z in 0..total_layers {
            let index = (now_z * 2) / 8;
            let in_index = now_z % 4;

            let byte = bytes[index];
            let valid = (byte >> (7 - in_index * 2)) & 1;
            let branch = (byte >> (6 - in_index * 2)) & 1;

            if valid == 1 {
                max_z = now_z as i32;
                // now_z の位置に branch を配置
                uxy |= (branch as u64) << now_z;
            }
        }

        // uxy を反転（ビットの並びを逆にする）
        let final_z = max_z as u8;
        let mut reversed_uxy = 0u64;
        for i in 0..=final_z {
            let bit = (uxy >> i) & 1;
            reversed_uxy |= bit << (final_z - i);
        }

        Segment {
            z: final_z,
            dim: reversed_uxy,
        }
    }
}
