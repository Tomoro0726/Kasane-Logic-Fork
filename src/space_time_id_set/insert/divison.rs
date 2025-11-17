use crate::{bit_vec::BitVec, space_time_id_set::SpaceTimeIdSet};

impl SpaceTimeIdSet {
    pub fn division(target: &BitVec, division: &mut Vec<BitVec>) -> Vec<BitVec> {
        if division.is_empty() {
            return vec![target.clone()];
        }

        let mut result = Vec::new();

        // まず pop した1つを使って最初の分割
        if let Some(first) = &mut division.pop() {
            let splitted = Self::split_dimension(target, first);
            result.extend(splitted);
        }

        // 残りの division について繰り返し分割
        while let Some(div) = &mut division.pop() {
            let mut new_result = Vec::new();

            for bit in result.into_iter() {
                let (start, end) = bit.under_prefix();
                let (d_start, d_end) = div.under_prefix();

                // 範囲が重なるなら分割
                if d_start < end && start < d_end {
                    let splitted = Self::split_dimension(&bit, div);
                    new_result.extend(splitted);
                } else {
                    // 重ならない場合そのまま残す
                    new_result.push(bit);
                }
            }

            result = new_result;
        }

        result
    }
}
