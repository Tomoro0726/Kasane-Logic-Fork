use std::collections::BTreeMap;

use crate::{
    bit_vec::BitVec,
    space_time_id_set::{LayerInfo, SpaceTimeIdSet, insert::insert_main_dim::DimensionSelect},
};

pub struct DimensionRefs<'a> {
    pub main: &'a BTreeMap<BitVec, LayerInfo>,
    pub a: &'a BTreeMap<BitVec, LayerInfo>,
    pub b: &'a BTreeMap<BitVec, LayerInfo>,
}

pub struct DimensionReverseRefs<'a> {
    pub main: &'a BTreeMap<BitVec, LayerInfo>,
    pub a: &'a BTreeMap<BitVec, LayerInfo>,
    pub b: &'a BTreeMap<BitVec, LayerInfo>,
}

impl SpaceTimeIdSet {
    /// メイン次元とその他の次元の参照を選択
    pub fn select_dimensions(&self, dim: &DimensionSelect) -> DimensionRefs<'_> {
        match dim {
            DimensionSelect::F => DimensionRefs {
                main: &self.f,
                a: &self.x,
                b: &self.y,
            },
            DimensionSelect::X => DimensionRefs {
                main: &self.x,
                a: &self.f,
                b: &self.y,
            },
            DimensionSelect::Y => DimensionRefs {
                main: &self.y,
                a: &self.f,
                b: &self.x,
            },
        }
    }
}
