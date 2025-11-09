use crate::{space_time_id_set::SpaceTimeIdSet, r#type::bit_vec::BitVec};

impl SpaceTimeIdSet {
    pub fn split_dimension(top: &BitVec, under: &mut BitVec) -> Vec<BitVec> {
        let mut result = vec![];

        while top != under {
            //最下層のBitを反転させる
            under.reverse_bottom_layer();
            result.push(under.clone());

            //最下層を削除する関数
            under.remove_bottom_layer();
        }

        result
    }
}
