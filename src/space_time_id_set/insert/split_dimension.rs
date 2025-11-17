use crate::{bit_vec::BitVec, space_time_id_set::SpaceTimeIdSet};
use std::fmt;

impl SpaceTimeIdSet {
    pub fn split_dimension(top: &BitVec, under: &mut BitVec) -> Vec<BitVec> {
        let mut result: Vec<BitVec> = vec![];

        println!("Top{}", top);
        println!("Under{}", under);

        // 無限ループ防止のための safety
        if under.0.len() < top.0.len() {
            panic!("under が top より小さすぎて top に到達できません");
        }

        while top != under {
            // under を変形しながら push
            under.reverse_bottom_layer();
            result.push(under.clone());
            under.remove_bottom_layer();

            // 無限ループ防止（top に絶対到達しない場合）
            if under.0.is_empty() {
                panic!("top に到達せず BitVec が空になりました");
            }
        }

        result
    }
}
