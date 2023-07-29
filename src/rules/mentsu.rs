/*!
 * Module for checking whether a collection of pais is a mentsu.
 */

use crate::*;

use itertools::Itertools;

/**
 * Whether a collection of tiles is a pair.
 */
pub fn is_jantou(tiles: impl IntoIterator<Item = Pai>) -> bool {
    let tiles_vec: Vec<Pai> = tiles.into_iter().collect();
    if tiles_vec.len() != 2 {
        return false;
    }
    let first = tiles_vec[0];
    let second = tiles_vec[1];
    return first == second;
}

/**
 * Whether a collection of tiles is a four-of-a-kind.
 */
pub fn is_kantsu(tiles: impl IntoIterator<Item = Pai>) -> bool {
    let tiles_vec: Vec<Pai> = tiles.into_iter().collect();
    if tiles_vec.len() != 4 {
        return false;
    }
    let first = tiles_vec[0];
    let second = tiles_vec[1];
    let third = tiles_vec[2];
    let fourth = tiles_vec[3];
    return first == second && first == third && first == fourth;
}

/**
 * Whether a collection of tiles is a three-of-a-kind.
 */
pub fn is_koutsu(tiles: impl IntoIterator<Item = Pai>) -> bool {
    let tiles_vec: Vec<Pai> = tiles.into_iter().collect();
    if tiles_vec.len() != 3 {
        return false;
    }
    let first = tiles_vec[0];
    let second = tiles_vec[1];
    let third = tiles_vec[2];
    return first == second && first == third;
}

/**
 * Whether a collection of tiles is a run of three.
 */
pub fn is_shuntsu(tiles: impl IntoIterator<Item = Pai>) -> bool {
    let tiles_vec: Vec<Pai> = tiles.into_iter().collect();
    if tiles_vec.len() != 3 {
        return false;
    }
    let first = tiles_vec[0];
    let second = tiles_vec[1];
    let third = tiles_vec[2];

    match first {
        Pai::Jihai(_) => return false,
        Pai::Suupai(tile1) => {
            match second {
                Pai::Jihai(_) => return false,
                Pai::Suupai(tile2) => {
                    match third {
                        Pai::Jihai(_) => return false,
                        Pai::Suupai(tile3) => {
                            if tile1.shoku != tile2.shoku ||
                                tile1.shoku != tile3.shoku {
                                return false;
                            }
                            let mut ranks = [tile1.rank, tile2.rank, tile3.rank];
                            ranks.sort();
                            return ranks[0] + 1 == ranks[1] && ranks[1] + 1 == ranks[2];
                        },
                    }
                },
            }
        },
    }
}

/**
 * Amount of times that the given tile repeats in the collection of tiles.
 */
fn count_repeats_of_pai(tile: Pai, ts: impl IntoIterator<Item = Pai>) -> usize {
    ts.into_iter().filter(|t| *t == tile).count()
}

fn possible_pair_pais(ts: impl IntoIterator<Item = Pai>) -> Vec<Pai> {
    let ts_vec: Vec<Pai> = ts.into_iter().collect();
    let unduped_tiles = ts_vec.iter().filter(|t| count_repeats_of_pai(**t, ts_vec.to_owned()) == 2);
    let unique_tiles = unduped_tiles.unique();
    let unique_vec: Vec<Pai> = unique_tiles.map(|t| t.clone()).collect();
    unique_vec
}

/**
 * Whether a hand is seven pairs.
 */
fn is_chiitoitsu(tiles: impl IntoIterator<Item = Pai>) -> bool {
    // TODO: Option to allow quads as two pairs.
    let tiles_vec: Vec<Pai> = tiles.into_iter().collect();
    if tiles_vec.len() != 14 { return false; }
    if possible_pair_pais(tiles_vec.to_owned()).len() != 7 { return false; }
    return true;
}

/**
 * Whether a hand is thirteen orphans.
 */
fn is_kokushi_musou(tiles: impl IntoIterator<Item = Pai>) -> bool {
    let tiles_vec: Vec<Pai> = tiles.into_iter().collect();
    if tiles_vec.len() != 14 { return false; }
    if possible_pair_pais(tiles_vec.to_owned()).len() != 1 { return false; }
    if !tiles_vec.contains(&Pai::Jihai(Jihai::Kazehai(Kazehai::Ton))) { return false; }
    if !tiles_vec.contains(&Pai::Jihai(Jihai::Kazehai(Kazehai::Nan))) { return false; }
    if !tiles_vec.contains(&Pai::Jihai(Jihai::Kazehai(Kazehai::Shaa))) { return false; }
    if !tiles_vec.contains(&Pai::Jihai(Jihai::Kazehai(Kazehai::Pei))) { return false; }
    if !tiles_vec.contains(&Pai::Jihai(Jihai::Sangenpai(Sangenpai::Chun))){  return false; }
    if !tiles_vec.contains(&Pai::Jihai(Jihai::Sangenpai(Sangenpai::Haku))) { return false; }
    if !tiles_vec.contains(&Pai::Jihai(Jihai::Sangenpai(Sangenpai::Hatsu))) { return false; }
    if !tiles_vec.contains(&Pai::Suupai(Suupai { shoku: Shoku::Manzu, rank: 1, akadora: false })) { return false; }
    if !tiles_vec.contains(&Pai::Suupai(Suupai { shoku: Shoku::Manzu, rank: 9, akadora: false })) { return false; }
    if !tiles_vec.contains(&Pai::Suupai(Suupai { shoku: Shoku::Pinzu, rank: 1, akadora: false })) { return false; }
    if !tiles_vec.contains(&Pai::Suupai(Suupai { shoku: Shoku::Pinzu, rank: 9, akadora: false })) { return false; }
    if !tiles_vec.contains(&Pai::Suupai(Suupai { shoku: Shoku::Souzu, rank: 1, akadora: false })) { return false; }
    if !tiles_vec.contains(&Pai::Suupai(Suupai { shoku: Shoku::Souzu, rank: 9, akadora: false })) { return false; }
    return true;
}

/**
 * Whether a collection of tiles is a complete hand.
 */
pub fn is_complete_hand(tiles: impl IntoIterator<Item = Pai>) -> bool {
    // TODO: melded groups

    fn could_start_shunstu(tile: Pai, ts: impl IntoIterator<Item = Pai>) -> bool {
        match tile {
            Pai::Jihai(_) => return false,
            Pai::Suupai(suupai) => {
                if suupai.rank > 7 {
                    return false;
                }
                let ts_vec: Vec<Pai> = ts.into_iter().collect();
                let mid = ts_vec.iter().find(|t| if let Pai::Suupai(s) = t {
                    s.shoku == suupai.shoku && s.rank == suupai.rank + 1
                } else { false });
                let high = ts_vec.iter().find(|t| if let Pai::Suupai(s) = t {
                    s.shoku == suupai.shoku && s.rank == suupai.rank + 2
                } else { false });
                return mid.is_some() && high.is_some();
            },
        }
    }

    fn remove_shuntsu(start: Pai, ts: impl IntoIterator<Item = Pai>) -> Vec<Pai> {
        if let Pai::Suupai(suupai) = start {
            let mut ts_vec: Vec<Pai> = ts.into_iter().collect();
            let low_index = ts_vec.to_owned().into_iter().position(|t| t == start);
            ts_vec.remove(low_index.unwrap());
            let mid_index = ts_vec.to_owned().into_iter().position(|t| t == Pai::Suupai(Suupai {
                shoku: suupai.shoku,
                rank: suupai.rank + 1,
                akadora: false,
            }));
            ts_vec.remove(mid_index.unwrap());
            let high_index = ts_vec.to_owned().into_iter().position(|t| t == Pai::Suupai(Suupai {
                shoku: suupai.shoku,
                rank: suupai.rank + 2,
                akadora: false,
            }));
            ts_vec.remove(high_index.unwrap());
            ts_vec
        } else {
            ts.into_iter().collect()
        }
    }

    fn possible_shuntsu_starts(ts: impl IntoIterator<Item = Pai>) -> Vec<Pai> {
        let ts_vec: Vec<Pai> = ts.into_iter().collect();
        let starts = ts_vec.iter().filter(|t| could_start_shunstu(**t, ts_vec.to_owned()));
        let starts_unique = starts.unique();
        let starts_vec: Vec<Pai> = starts_unique.map(|t| t.clone()).collect();
        starts_vec
    }

    fn possible_quad_pais(ts: impl IntoIterator<Item = Pai>) -> Vec<Pai> {
        let ts_vec: Vec<Pai> = ts.into_iter().collect();
        let unduped_tiles = ts_vec.iter().filter(|t| count_repeats_of_pai(**t, ts_vec.to_owned()) == 4);
        let unique_tiles = unduped_tiles.unique();
        let unique_vec: Vec<Pai> = unique_tiles.map(|t| t.clone()).collect();
        unique_vec
    }

    fn possible_trip_pais(ts: impl IntoIterator<Item = Pai>) -> Vec<Pai> {
        let ts_vec: Vec<Pai> = ts.into_iter().collect();
        let unduped_tiles = ts_vec.iter().filter(|t| count_repeats_of_pai(**t, ts_vec.to_owned()) == 3);
        let unique_tiles = unduped_tiles.unique();
        let unique_vec: Vec<Pai> = unique_tiles.map(|t| t.clone()).collect();
        unique_vec
    }

    // Whether the remaining pais form a complete hand with the pais put aside.
    fn remaining_pais_complete_hand(
        ts: impl IntoIterator<Item = Pai>,
        possible_quad_tiles: impl IntoIterator<Item = Pai> + Clone,
        possible_trip_tiles: impl IntoIterator<Item = Pai> + Clone,
        possible_shuntsu_tiles: impl IntoIterator<Item = Pai> + Clone,
        amt_pais_removed: u8, 
        amt_quads_removed: u8
    ) -> bool {
        let ts_vec: Vec<Pai> = ts.into_iter().collect();

        // For each quad candidate, try including and omitting it.
        let quad_tiles: Vec<Pai> = possible_quad_tiles.to_owned().into_iter().collect();
        if let Some((head, tail)) = quad_tiles.split_first() {
            let tiles_left_in_hand: Vec<Pai> = ts_vec.to_owned().into_iter().filter(|t| *t != *head).collect();
            let remaining_quad_candidates = tail.to_vec();
            if remaining_pais_complete_hand(
                tiles_left_in_hand,
                remaining_quad_candidates.to_owned(),
                possible_trip_tiles.to_owned().into_iter().filter(|t| *t != *head).collect_vec(),
                possible_shuntsu_tiles.to_owned().into_iter().filter(|t| *t != *head).collect_vec(),
                amt_pais_removed + 4,
                amt_quads_removed + 1
            ) {
                return true;
            }
            if remaining_pais_complete_hand(
                ts_vec.to_owned(),
                remaining_quad_candidates,
                possible_trip_tiles.to_owned(),
                possible_shuntsu_tiles.to_owned(),
                amt_pais_removed,
                amt_quads_removed
            ) {
                return true;
            }
        }

        // For each trip candidate, try including and omitting it.
        let trip_tiles: Vec<Pai> = possible_trip_tiles.to_owned().into_iter().collect();
        if let Some((head, tail)) = trip_tiles.split_first() {
            let tiles_left_in_hand: Vec<Pai> = ts_vec.to_owned().into_iter().filter(|t| *t != *head).collect();
            let remaining_trip_candidates = tail.to_vec();
            if remaining_pais_complete_hand(
                tiles_left_in_hand.to_owned(),
                possible_quad_tiles.to_owned().into_iter().filter(|t| *t != *head).collect_vec(),
                remaining_trip_candidates.to_owned(),
                possible_shuntsu_starts(tiles_left_in_hand),
                amt_pais_removed + 3,
                amt_quads_removed
            ) {
                return true;
            }
            if remaining_pais_complete_hand(
                ts_vec.to_owned(),
                possible_quad_tiles.to_owned(),
                remaining_trip_candidates,
                possible_shuntsu_tiles.to_owned(),
                amt_pais_removed,
                amt_quads_removed
            ) {
                return true;
            }
        }

        // For each shuntsu candidate, try including and omitting it.
        let shuntsu_starts: Vec<Pai> = possible_shuntsu_tiles.into_iter().collect();
        if let Some((head, tail)) = shuntsu_starts.split_first() {
            let tiles_left_in_hand: Vec<Pai> = remove_shuntsu(*head, ts_vec.to_owned());
            let remaining_shuntsu_candidates: Vec<Pai> = tail.iter().filter(|t| could_start_shunstu(**t, tiles_left_in_hand.to_owned())).map(|t| *t).collect();
            let first_case_candidates: Vec<Pai> = if could_start_shunstu(*head, tiles_left_in_hand.to_owned()) {
                // If the tile can still start another run then reconsider it in the first case.
                let mut copy = remaining_shuntsu_candidates.to_owned();
                copy.push(*head);
                copy
            } else {
                remaining_shuntsu_candidates.to_owned()
            };
            if remaining_pais_complete_hand(
                tiles_left_in_hand.to_owned(),
                possible_quad_tiles.to_owned().into_iter().filter(|t| tiles_left_in_hand.contains(t)).collect_vec(),
                possible_trip_tiles.to_owned().into_iter().filter(|t| tiles_left_in_hand.contains(t)).collect_vec(),
                first_case_candidates.to_owned(),
                amt_pais_removed + 3,
                amt_quads_removed
            ) {
                return true;
            }
            if remaining_pais_complete_hand(
                ts_vec.to_owned(),
                possible_quad_tiles,
                possible_trip_tiles,
                remaining_shuntsu_candidates,
                amt_pais_removed,
                amt_quads_removed
            ) {
                return true;
            }
        }

        return is_jantou(ts_vec) && amt_pais_removed == 12 + amt_quads_removed;
    }

    let tiles_vec: Vec<Pai> = tiles.into_iter().collect();
    if is_chiitoitsu(tiles_vec.to_owned()) || is_kokushi_musou(tiles_vec.to_owned()) {
        return true;
    }
    let possible_quad_tiles = possible_quad_pais(tiles_vec.to_owned());
    let possible_trip_tiles = possible_trip_pais(tiles_vec.to_owned());
    let possible_shuntsu_tiles = possible_shuntsu_starts(tiles_vec.to_owned());
    return remaining_pais_complete_hand(
        tiles_vec,
        possible_quad_tiles,
        possible_trip_tiles,
        possible_shuntsu_tiles,
        0,
        0
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_red_dragons_are_a_pair() {
        let tile1 = Pai::Jihai(Jihai::Sangenpai(Sangenpai::Chun));
        let tile2 = Pai::Jihai(Jihai::Sangenpai(Sangenpai::Chun));
        assert_eq!(
            is_jantou(vec![tile1, tile2]),
            true
        );
    }

    #[test]
    fn two_threes_of_bamboos_are_a_pair() {
        let tile1 = Pai::Suupai(Suupai { shoku: Shoku::Souzu, rank: 3, akadora: false });
        let tile2 = Pai::Suupai(Suupai { shoku: Shoku::Souzu, rank: 3, akadora: false });
        assert_eq!(
            is_jantou(vec![tile1, tile2]),
            true
        );
    }

    #[test]
    fn two_different_threes_are_not_a_pair() {
        let tile1 = Pai::Suupai(Suupai { shoku: Shoku::Souzu, rank: 3, akadora: false });
        let tile2 = Pai::Suupai(Suupai { shoku: Shoku::Manzu, rank: 3, akadora: false });
        assert_eq!(
            is_jantou(vec![tile1, tile2]),
            false
        );
    }

    #[test]
    fn three_north_winds_are_a_triplet() {
        let tile1 = Pai::Jihai(Jihai::Kazehai(Kazehai::Pei));
        let tile2 = Pai::Jihai(Jihai::Kazehai(Kazehai::Pei));
        let tile3 = Pai::Jihai(Jihai::Kazehai(Kazehai::Pei));
        assert_eq!(
            is_koutsu(vec![tile1, tile2, tile3]),
            true
        );
    }

    #[test]
    fn three_different_winds_are_a_not_triplet() {
        let tile1 = Pai::Jihai(Jihai::Kazehai(Kazehai::Pei));
        let tile2 = Pai::Jihai(Jihai::Kazehai(Kazehai::Ton));
        let tile3 = Pai::Jihai(Jihai::Kazehai(Kazehai::Shaa));
        assert_eq!(
            is_koutsu(vec![tile1, tile2, tile3]),
            false
        );
    }

    #[test]
    fn single_tile_is_not_triplet() {
        let tile1 = Pai::Jihai(Jihai::Kazehai(Kazehai::Pei));
        assert_eq!(
            is_koutsu(vec![tile1]),
            false
        );
    }

    #[test]
    fn four_five_of_bamboos_are_a_quad() {
        let tile1 = Pai::Suupai(Suupai { shoku: Shoku::Pinzu, rank: 3, akadora: false });
        let tile2 = Pai::Suupai(Suupai { shoku: Shoku::Pinzu, rank: 1, akadora: false });
        let tile3 = Pai::Suupai(Suupai { shoku: Shoku::Pinzu, rank: 2, akadora: false });
        assert_eq!(
            is_shuntsu(vec![tile1, tile2, tile3]),
            true
        );
    }

    #[test]
    fn one_two_three_of_dots_is_a_run() {
        let tile1 = Pai::Suupai(Suupai { shoku: Shoku::Souzu, rank: 5, akadora: false });
        let tile2 = Pai::Suupai(Suupai { shoku: Shoku::Souzu, rank: 5, akadora: false });
        let tile3 = Pai::Suupai(Suupai { shoku: Shoku::Souzu, rank: 5, akadora: false });
        let tile4 = Pai::Suupai(Suupai { shoku: Shoku::Souzu, rank: 5, akadora: true });
        assert_eq!(
            is_kantsu(vec![tile1, tile2, tile3, tile4]),
            true
        );
    }

    #[test]
    fn one_tile_is_not_a_complete_hand() {
        assert_eq!(
            is_complete_hand(vec![
                Pai::Jihai(Jihai::Kazehai(Kazehai::Ton)),
            ]),
            false
        );
    }

    #[test]
    fn fourteen_arbitrary_tiles_are_not_a_complete_hand() {
        assert_eq!(
            is_complete_hand(vec![
                Pai::Jihai(Jihai::Kazehai(Kazehai::Ton)),
                Pai::Jihai(Jihai::Kazehai(Kazehai::Nan)),
                Pai::Jihai(Jihai::Kazehai(Kazehai::Nan)),
                Pai::Jihai(Jihai::Kazehai(Kazehai::Shaa)),
                Pai::Jihai(Jihai::Kazehai(Kazehai::Pei)),
                Pai::Jihai(Jihai::Sangenpai(Sangenpai::Chun)),
                Pai::Jihai(Jihai::Sangenpai(Sangenpai::Chun)),
                Pai::Suupai(Suupai { shoku: Shoku::Manzu, rank: 3, akadora: false }),
                Pai::Suupai(Suupai { shoku: Shoku::Manzu, rank: 3, akadora: false }),
                Pai::Suupai(Suupai { shoku: Shoku::Manzu, rank: 3, akadora: false }),
                Pai::Suupai(Suupai { shoku: Shoku::Manzu, rank: 5, akadora: true }),
                Pai::Suupai(Suupai { shoku: Shoku::Pinzu, rank: 5, akadora: true }),
                Pai::Suupai(Suupai { shoku: Shoku::Pinzu, rank: 7, akadora: false }),
                Pai::Suupai(Suupai { shoku: Shoku::Pinzu, rank: 6, akadora: false }),
            ]),
            false
        );
    }

    #[test]
    fn thirteen_orphans_is_a_complete_hand() {
        assert_eq!(
            is_complete_hand(vec![
                Pai::Jihai(Jihai::Kazehai(Kazehai::Ton)),
                Pai::Jihai(Jihai::Kazehai(Kazehai::Ton)),
                Pai::Jihai(Jihai::Kazehai(Kazehai::Nan)),
                Pai::Jihai(Jihai::Kazehai(Kazehai::Shaa)),
                Pai::Jihai(Jihai::Kazehai(Kazehai::Pei)),
                Pai::Jihai(Jihai::Sangenpai(Sangenpai::Chun)),
                Pai::Jihai(Jihai::Sangenpai(Sangenpai::Haku)),
                Pai::Jihai(Jihai::Sangenpai(Sangenpai::Hatsu)),
                Pai::Suupai(Suupai { shoku: Shoku::Manzu, rank: 1, akadora: false }),
                Pai::Suupai(Suupai { shoku: Shoku::Manzu, rank: 9, akadora: false }),
                Pai::Suupai(Suupai { shoku: Shoku::Pinzu, rank: 1, akadora: false }),
                Pai::Suupai(Suupai { shoku: Shoku::Pinzu, rank: 9, akadora: false }),
                Pai::Suupai(Suupai { shoku: Shoku::Souzu, rank: 1, akadora: false }),
                Pai::Suupai(Suupai { shoku: Shoku::Souzu, rank: 9, akadora: false }),
            ]),
            true
        );
    }

    #[test]
    fn seven_pairs_is_a_complete_hand() {
        assert_eq!(
            is_complete_hand(vec![
                Pai::Jihai(Jihai::Kazehai(Kazehai::Ton)),
                Pai::Jihai(Jihai::Kazehai(Kazehai::Ton)),
                Pai::Jihai(Jihai::Kazehai(Kazehai::Nan)),
                Pai::Jihai(Jihai::Kazehai(Kazehai::Nan)),
                Pai::Jihai(Jihai::Kazehai(Kazehai::Pei)),
                Pai::Jihai(Jihai::Kazehai(Kazehai::Pei)),
                Pai::Jihai(Jihai::Sangenpai(Sangenpai::Chun)),
                Pai::Jihai(Jihai::Sangenpai(Sangenpai::Chun)),
                Pai::Jihai(Jihai::Sangenpai(Sangenpai::Hatsu)),
                Pai::Jihai(Jihai::Sangenpai(Sangenpai::Hatsu)),
                Pai::Suupai(Suupai { shoku: Shoku::Manzu, rank: 1, akadora: false }),
                Pai::Suupai(Suupai { shoku: Shoku::Manzu, rank: 1, akadora: false }),
                Pai::Suupai(Suupai { shoku: Shoku::Pinzu, rank: 9, akadora: false }),
                Pai::Suupai(Suupai { shoku: Shoku::Pinzu, rank: 9, akadora: false }),
            ]),
            true
        );
    }

    #[test]
    fn big_four_winds_is_a_complete_hand() {
        assert_eq!(
            is_complete_hand(vec![
                Pai::Jihai(Jihai::Kazehai(Kazehai::Ton)),
                Pai::Jihai(Jihai::Kazehai(Kazehai::Ton)),
                Pai::Jihai(Jihai::Kazehai(Kazehai::Ton)),
                Pai::Jihai(Jihai::Kazehai(Kazehai::Ton)),
                Pai::Jihai(Jihai::Kazehai(Kazehai::Nan)),
                Pai::Jihai(Jihai::Kazehai(Kazehai::Nan)),
                Pai::Jihai(Jihai::Kazehai(Kazehai::Nan)),
                Pai::Jihai(Jihai::Kazehai(Kazehai::Nan)),
                Pai::Jihai(Jihai::Kazehai(Kazehai::Shaa)),
                Pai::Jihai(Jihai::Kazehai(Kazehai::Shaa)),
                Pai::Jihai(Jihai::Kazehai(Kazehai::Shaa)),
                Pai::Jihai(Jihai::Kazehai(Kazehai::Shaa)),
                Pai::Jihai(Jihai::Kazehai(Kazehai::Pei)),
                Pai::Jihai(Jihai::Kazehai(Kazehai::Pei)),
                Pai::Jihai(Jihai::Kazehai(Kazehai::Pei)),
                Pai::Jihai(Jihai::Kazehai(Kazehai::Pei)),
                Pai::Jihai(Jihai::Sangenpai(Sangenpai::Chun)),
                Pai::Jihai(Jihai::Sangenpai(Sangenpai::Chun)),
            ]),
            true
        );
    }

    #[test]
    fn all_trips_and_quads_is_a_complete_hand() {
        assert_eq!(
            is_complete_hand(vec![
                Pai::Jihai(Jihai::Kazehai(Kazehai::Ton)),
                Pai::Jihai(Jihai::Kazehai(Kazehai::Ton)),
                Pai::Jihai(Jihai::Kazehai(Kazehai::Ton)),
                Pai::Jihai(Jihai::Kazehai(Kazehai::Nan)),
                Pai::Jihai(Jihai::Kazehai(Kazehai::Nan)),
                Pai::Jihai(Jihai::Kazehai(Kazehai::Nan)),
                Pai::Jihai(Jihai::Kazehai(Kazehai::Nan)),
                Pai::Jihai(Jihai::Kazehai(Kazehai::Shaa)),
                Pai::Jihai(Jihai::Kazehai(Kazehai::Shaa)),
                Pai::Jihai(Jihai::Kazehai(Kazehai::Shaa)),
                Pai::Jihai(Jihai::Kazehai(Kazehai::Pei)),
                Pai::Jihai(Jihai::Kazehai(Kazehai::Pei)),
                Pai::Jihai(Jihai::Kazehai(Kazehai::Pei)),
                Pai::Jihai(Jihai::Kazehai(Kazehai::Pei)),
                Pai::Jihai(Jihai::Sangenpai(Sangenpai::Chun)),
                Pai::Jihai(Jihai::Sangenpai(Sangenpai::Chun)),
            ]),
            true
        );
    }

    #[test]
    fn all_sequences_is_a_complete_hand() {
        assert_eq!(
            is_complete_hand(vec![
                Pai::Jihai(Jihai::Kazehai(Kazehai::Nan)),
                Pai::Jihai(Jihai::Kazehai(Kazehai::Nan)),
                Pai::Suupai(Suupai { shoku: Shoku::Manzu, rank: 3, akadora: false }),
                Pai::Suupai(Suupai { shoku: Shoku::Manzu, rank: 4, akadora: false }),
                Pai::Suupai(Suupai { shoku: Shoku::Manzu, rank: 2, akadora: false }),
                Pai::Suupai(Suupai { shoku: Shoku::Pinzu, rank: 5, akadora: true }),
                Pai::Suupai(Suupai { shoku: Shoku::Pinzu, rank: 7, akadora: false }),
                Pai::Suupai(Suupai { shoku: Shoku::Pinzu, rank: 6, akadora: false }),
                Pai::Suupai(Suupai { shoku: Shoku::Pinzu, rank: 5, akadora: false }),
                Pai::Suupai(Suupai { shoku: Shoku::Pinzu, rank: 4, akadora: false }),
                Pai::Suupai(Suupai { shoku: Shoku::Pinzu, rank: 3, akadora: false }),
                Pai::Suupai(Suupai { shoku: Shoku::Pinzu, rank: 5, akadora: false }),
                Pai::Suupai(Suupai { shoku: Shoku::Pinzu, rank: 4, akadora: false }),
                Pai::Suupai(Suupai { shoku: Shoku::Pinzu, rank: 3, akadora: false }),
            ]),
            true
        );
    }

    #[test]
    fn mix_of_sets_and_sequences_is_a_complete_hand() {
        assert_eq!(
            is_complete_hand(vec![
                Pai::Jihai(Jihai::Kazehai(Kazehai::Ton)),
                Pai::Jihai(Jihai::Kazehai(Kazehai::Ton)),
                Pai::Jihai(Jihai::Kazehai(Kazehai::Ton)),
                Pai::Jihai(Jihai::Kazehai(Kazehai::Pei)),
                Pai::Jihai(Jihai::Kazehai(Kazehai::Pei)),
                Pai::Jihai(Jihai::Kazehai(Kazehai::Pei)),
                Pai::Jihai(Jihai::Kazehai(Kazehai::Pei)),
                Pai::Jihai(Jihai::Sangenpai(Sangenpai::Chun)),
                Pai::Jihai(Jihai::Sangenpai(Sangenpai::Chun)),
                Pai::Suupai(Suupai { shoku: Shoku::Manzu, rank: 3, akadora: false }),
                Pai::Suupai(Suupai { shoku: Shoku::Manzu, rank: 4, akadora: false }),
                Pai::Suupai(Suupai { shoku: Shoku::Manzu, rank: 2, akadora: false }),
                Pai::Suupai(Suupai { shoku: Shoku::Pinzu, rank: 5, akadora: true }),
                Pai::Suupai(Suupai { shoku: Shoku::Pinzu, rank: 7, akadora: false }),
                Pai::Suupai(Suupai { shoku: Shoku::Pinzu, rank: 6, akadora: false }),
            ]),
            true
        );
    }
}
