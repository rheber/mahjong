/*!
 * Module for checking whether a collection of pais is a mentsu.
 */

use crate::*;

/**
 * Whether a collection of tiles is a pair.
 */
pub fn is_jantou(tiles: &[Pai]) -> bool {
    if tiles.len() != 2 {
        return false;
    }
    let first = &tiles[0];
    let second = &tiles[1];
    return first == second;
}

/**
 * Whether a collection of tiles is a four-of-a-kind.
 */
pub fn is_kantsu(tiles: &[Pai]) -> bool {
    if tiles.len() != 4 {
        return false;
    }
    let first = &tiles[0];
    let second = &tiles[1];
    let third = &tiles[2];
    let fourth = &tiles[3];
    return first == second && first == third && first == fourth;
}

/**
 * Whether a collection of tiles is a three-of-a-kind.
 */
pub fn is_koutsu(tiles: &[Pai]) -> bool {
    if tiles.len() != 3 {
        return false;
    }
    let first = &tiles[0];
    let second = &tiles[1];
    let third = &tiles[2];
    return first == second && first == third;
}

/**
 * Whether a collection of tiles is a run of three.
 */
pub fn is_shuntsu(tiles: &[Pai]) -> bool {
    if tiles.len() != 3 {
        return false;
    }
    let first = &tiles[0];
    let second = &tiles[1];
    let third = &tiles[2];

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
pub fn is_complete_hand(tiles: &[Pai]) -> bool {
    // TODO: seven pairs and thirteen orphans

    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_red_dragons_are_a_pair() {
        let tile1 = Pai::Jihai(Jihai::Sangenpai(Sangenpai::Chun));
        let tile2 = Pai::Jihai(Jihai::Sangenpai(Sangenpai::Chun));
        assert_eq!(
            is_jantou(&[tile1, tile2]),
            true
        );
    }

    #[test]
    fn two_threes_of_bamboos_are_a_pair() {
        let tile1 = Pai::Suupai(Suupai { shoku: Shoku::Souzu, rank: 3, akadora: false });
        let tile2 = Pai::Suupai(Suupai { shoku: Shoku::Souzu, rank: 3, akadora: false });
        assert_eq!(
            is_jantou(&[tile1, tile2]),
            true
        );
    }

    #[test]
    fn two_different_threes_are_not_a_pair() {
        let tile1 = Pai::Suupai(Suupai { shoku: Shoku::Souzu, rank: 3, akadora: false });
        let tile2 = Pai::Suupai(Suupai { shoku: Shoku::Manzu, rank: 3, akadora: false });
        assert_eq!(
            is_jantou(&[tile1, tile2]),
            false
        );
    }

    #[test]
    fn three_north_winds_are_a_triplet() {
        let tile1 = Pai::Jihai(Jihai::Kazehai(Kazehai::Pei));
        let tile2 = Pai::Jihai(Jihai::Kazehai(Kazehai::Pei));
        let tile3 = Pai::Jihai(Jihai::Kazehai(Kazehai::Pei));
        assert_eq!(
            is_koutsu(&[tile1, tile2, tile3]),
            true
        );
    }

    #[test]
    fn three_different_winds_are_a_not_triplet() {
        let tile1 = Pai::Jihai(Jihai::Kazehai(Kazehai::Pei));
        let tile2 = Pai::Jihai(Jihai::Kazehai(Kazehai::Ton));
        let tile3 = Pai::Jihai(Jihai::Kazehai(Kazehai::Shaa));
        assert_eq!(
            is_koutsu(&[tile1, tile2, tile3]),
            false
        );
    }

    #[test]
    fn single_tile_is_not_triplet() {
        let tile1 = Pai::Jihai(Jihai::Kazehai(Kazehai::Pei));
        assert_eq!(
            is_koutsu(&[tile1]),
            false
        );
    }

    #[test]
    fn four_five_of_bamboos_are_a_quad() {
        let tile1 = Pai::Suupai(Suupai { shoku: Shoku::Pinzu, rank: 3, akadora: false });
        let tile2 = Pai::Suupai(Suupai { shoku: Shoku::Pinzu, rank: 1, akadora: false });
        let tile3 = Pai::Suupai(Suupai { shoku: Shoku::Pinzu, rank: 2, akadora: false });
        assert_eq!(
            is_shuntsu(&[tile1, tile2, tile3]),
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
            is_kantsu(&[tile1, tile2, tile3, tile4]),
            true
        );
    }
}
