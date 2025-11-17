use crate::{
    bit_vec::BitVec,
    space_time_id_set::{Index, SpaceTimeIdSet, insert::insert_main_dim::DimensionSelect},
};
impl SpaceTimeIdSet {
    ///相手を切断する
    pub fn top_top_under(
        &mut self,
        target_index: Index,
        target_bit: BitVec,
        target_dim: DimensionSelect,
    ) {
        let reverse = self.reverse.remove(&target_index).unwrap();

        //引かれる部分
        let top = match target_dim {
            DimensionSelect::F => &reverse.f,
            DimensionSelect::X => &reverse.x,
            DimensionSelect::Y => &reverse.y,
        };

        let splited = BitVec::division(top.clone(), vec![target_bit]);

        for single in splited {
            match target_dim {
                DimensionSelect::F => {
                    self.uncheck_insert(&single, &reverse.x, &reverse.y);
                }
                DimensionSelect::X => {
                    self.uncheck_insert(&reverse.f, &single, &reverse.y);
                }
                DimensionSelect::Y => {
                    self.uncheck_insert(&reverse.f, &reverse.x, &single);
                }
            }
        }
    }
}
