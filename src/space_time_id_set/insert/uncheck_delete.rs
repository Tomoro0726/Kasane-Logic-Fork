use crate::space_time_id_set::{Index, SpaceTimeIdSet};

impl SpaceTimeIdSet {
    pub fn uncheck_delete(&mut self, index: &Index) {
        let removed = self.reverse.remove(index).unwrap();

        self.f.remove(&removed.f);
        self.x.remove(&removed.x);
        self.y.remove(&removed.y);
    }
}
