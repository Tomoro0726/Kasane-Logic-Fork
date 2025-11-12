use std::collections::HashSet;

use crate::{
    bit_vec::BitVec,
    space_time_id_set::{Index, SpaceTimeIdSet, insert::insert_main_dim::MainDimensionSelect},
};

use std::ops::Bound::{Excluded, Included};

impl SpaceTimeIdSet {
    ///与えられた次元において、下位の範囲を収集する
    pub fn collect_under(
        &self,
        main_bit: &BitVec,
        main_dim_select: &MainDimensionSelect,
    ) -> HashSet<Index> {
        let mut main_under = HashSet::new();

        let dims = self.select_dimensions(&main_dim_select);

        let (start, end) = match *main_bit < main_bit.under_prefix() {
            true => (main_bit.clone(), main_bit.under_prefix()),
            false => (main_bit.under_prefix(), main_bit.clone()),
        };

        for (_, layerinfo) in dims.main.range((Excluded(start), Excluded(end))) {
            main_under.extend(layerinfo.index.clone());
        }

        main_under
    }
}
