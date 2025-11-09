use std::collections::HashSet;

use crate::{
    space_time_id_set::{
        Index, SpaceTimeIdSet,
        insert::{
            check_relation::{self, Relation},
            insert_main_dim::MainDimensionSelect,
        },
    },
    r#type::bit_vec::BitVec,
};

impl SpaceTimeIdSet {
    pub fn scan_and_insert_top(
        &mut self,
        main_bit: &BitVec,
        main_top: &HashSet<Index>,
        other_encoded: &[&Vec<(usize, BitVec)>; 2],
        main_dim_select: MainDimensionSelect,
    ) {
        //徐々に削除していくので配列をコピーする必要がありそう
        let mut other_encoded_copy = other_encoded.map(|v| (*v).clone());

        for index in main_top {
            let reverse = self.reverse.get(index).unwrap();
            let target_x = &reverse.x.clone();
            let target_y = &reverse.y.clone();

            match main_dim_select {
                MainDimensionSelect::F => {
                    let mut x_relations = Vec::new();
                    let mut y_relations = Vec::new();

                    // Xを処理
                    for (x_index, (_, x_bit)) in other_encoded[0].iter().enumerate() {
                        let relation = Self::check_relation(x_bit, &target_x);

                        // 無関係な場合の処理を早期continueで分離
                        if relation == check_relation::Relation::Disjoint {
                            let x_bit_removed = other_encoded_copy[0].remove(x_index);

                            // Y方向のすべてに uncheck_insert
                            for (_, y_bit) in &other_encoded_copy[1] {
                                self.uncheck_insert(main_bit, &x_bit_removed.1, y_bit);
                            }

                            //無関係の場合はx_relationsにpushしない
                            continue;
                        }

                        // 関係あり → 後で使うため保存
                        x_relations.push((x_index, relation));
                    }

                    // Yを処理
                    for (y_index, (_, y_bit)) in other_encoded[1].iter().enumerate() {
                        y_relations.push((y_index, Self::check_relation(y_bit, &target_y)));

                        let relation = Self::check_relation(y_bit, &target_y);

                        // 無関係な場合の処理を早期continueで分離
                        if relation == check_relation::Relation::Disjoint {
                            let y_bit_removed = other_encoded_copy[1].remove(y_index);

                            // Y方向のすべてに uncheck_insert
                            for (_, x_bit) in &other_encoded_copy[0] {
                                self.uncheck_insert(main_bit, &y_bit_removed.1, x_bit);
                            }

                            //無関係の場合はx_relationsにpushしない
                            continue;
                        }

                        // 関係あり → 後で使うため保存
                        y_relations.push((y_index, relation));
                    }

                    //F,X,Yの全てにおいて関連があったものを処理していく
                    for (x_index, x_relation) in &x_relations {
                        for (y_index, y_relation) in &y_relations {
                            //ここではFは必ず上位

                            //X=Top|Y=Topの場合は相手を切る
                            //そのIndexのIDを削除して新しいIDを1つ以上追加する
                            if *x_relation == Relation::Top {}

                            if *y_relation == Relation::Top {}

                            //その他の場合は自分を削って再帰的に挿入
                        }
                    }
                }
                MainDimensionSelect::X => todo!(),
                MainDimensionSelect::Y => todo!(),
            }
        }
    }
}
