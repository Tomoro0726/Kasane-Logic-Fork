use crate::bit_vec::BitVec;

impl BitVec {
    /// 下位範囲を検索するときに必要な右側の端点の値を出す
    /// (Start, End) ただし Start < End
    pub fn under_prefix(&self) -> (BitVec, BitVec) {
        let a = self.clone();
        let mut b = self.clone();

        // b.0 が Vec<u8> であると仮定
        if let Some(last) = b.0.last_mut() {
            // 下位4組(0〜6bit)の2bit単位で探索
            for i in 0..=3 {
                let mask: u8 = 0b00000011 << (i * 2);

                if *last & mask != 0 {
                    // その位置の下位ビットを反転
                    let mask_split: u8 = 0b00000001 << (i * 2);
                    *last ^= mask_split;
                    break;
                }
            }
        }

        if a < b { (a, b) } else { (b, a) }
    }
}
