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
 * Whether a collection of tiles is a complete hand.
 */
pub fn is_complete_hand(tiles: impl IntoIterator<Item = Pai>) -> bool {
    // TODO: seven pairs and thirteen orphans
    // TODO: runs and triplets

    fn count_pai_in_pais(tile: Pai, ts: impl IntoIterator<Item = Pai>) -> usize {
        ts.into_iter().filter(|t| *t == tile).count()
    }

    fn possible_quad_pais(ts: impl IntoIterator<Item = Pai>) -> impl Iterator<Item = Pai> {
        let ts_vec: Vec<Pai> = ts.into_iter().collect();
        let unduped_tiles = ts_vec.iter().filter(|t| count_pai_in_pais(**t, ts_vec.to_owned()) == 4);
        let unique_tiles = unduped_tiles.unique();
        let unique_vec: Vec<Pai> = unique_tiles.map(|t| t.clone()).collect();
        unique_vec.into_iter()
    }

    fn remaining_pais_complete_hand(
        ts: impl IntoIterator<Item = Pai>,
        possible_quad_tiles: impl IntoIterator<Item = Pai>,
        amt_pais_removed: u8, 
        amt_quads_removed: u8
    ) -> bool {
        let ts_vec: Vec<Pai> = ts.into_iter().collect();
        let quad_tiles: Vec<Pai> = possible_quad_tiles.into_iter().collect();
        match quad_tiles.split_first() {
            Some((head, tail)) => {
                let new_tiles: Vec<Pai> = ts_vec.to_owned().into_iter().filter(|t| *t != *head).collect();
                let tail_vec = tail.to_vec();
                if remaining_pais_complete_hand(new_tiles, tail_vec.to_owned(), amt_pais_removed + 4, amt_quads_removed + 1) {
                    return true;
                }
                if remaining_pais_complete_hand(ts_vec.to_owned(), tail_vec, amt_pais_removed, amt_quads_removed) {
                    return true;
                }
            },
            None => {
                // continue
            },
        }
        return is_jantou(ts_vec) && amt_pais_removed == 12 + amt_quads_removed;
    }

    let tiles_vec: Vec<Pai> = tiles.into_iter().collect();
    let possible_quad_tiles = possible_quad_pais(tiles_vec.to_owned());
    return remaining_pais_complete_hand(tiles_vec, possible_quad_tiles, 0, 0);
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
}
