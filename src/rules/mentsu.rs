/*!
 * Module for checking whether a collection of pais is a mentsu.
 */

use crate::*;

/**
 * Whether a list of tiles is a pair.
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
 * Whether a list of tiles is a three-of-a-kind.
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
    fn two_different_threes_are_not_a_pair() {
        let tile1 = Pai::Suupai(Suupai { shoku: Shoku::Souzu, rank: 3, akadora: false });
        let tile2 = Pai::Suupai(Suupai { shoku: Shoku::Manzu, rank: 3, akadora: false });
        assert_eq!(
            is_jantou(&[tile1, tile2]),
            false
        );
    }
}
