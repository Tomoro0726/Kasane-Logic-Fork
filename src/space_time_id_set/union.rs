use crate::space_time_id_set::{SpaceTimeIdSet, insert::select_dimensions::DimensionSelect};

impl SpaceTimeIdSet {
    ///二つのSpaceTimeIdSetを結合する
    pub fn union(&self, other: &SpaceTimeIdSet) -> SpaceTimeIdSet {
        //短いSetを長いSetに挿入する

        if self.iter().len() > other.iter().len() {
            let mut copy = self.clone();

            for (_, reverse) in other.reverse.clone() {
                copy.uncheck_insert(&reverse.f, &reverse.x, &reverse.y, &DimensionSelect::F);
            }

            return copy;
        } else {
            let mut copy = other.clone();

            for (_, reverse) in self.reverse.clone() {
                copy.uncheck_insert(&reverse.f, &reverse.x, &reverse.y, &DimensionSelect::F);
            }

            return copy;
        }
    }
}
