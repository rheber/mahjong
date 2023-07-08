use regex::Regex;

/**
 * A suit.
 */
#[derive(PartialEq)]
pub enum Shoku {
    Manzu,
    Pinzu,
    Souzu,
}

/**
 * A wind.
 */
#[derive(PartialEq)]
pub enum Kazehai {
    Ton,
    Nan,
    Shaa,
    Pei,
}

/**
 * A dragon.
 */
#[derive(PartialEq)]
pub enum Sangenpai {
    Chun,
    Haku,
    Hatsu,
}

/**
 * A number tile.
 */
#[derive(PartialEq)]
pub struct Suupai {
    /**
     * The suit.
     */
    shoku: Shoku,

    /**
     * The rank, a number from 1 through 9.
     */
    rank: i8,

    /**
     * Whether this tile is a red dora.
     */
    akadora: bool,
}

/**
 * An honor tile.
 */
#[derive(PartialEq)]
pub enum Jihai {
    Kazehai(Kazehai),
    Sangenpai(Sangenpai),
}

/**
 * A tile.
 */
#[derive(PartialEq)]
pub enum Pai {
    Jihai(Jihai),
    Suupai(Suupai),
}

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

/**
 * Whether the given candidate is a valid tilestring.
 * m/w/c -> manzu
 * p/d -> pinzu
 * s/b -> souzu
 * z/h -> jihai
 * 0 -> red five
 * 1z -> ton
 * 5z -> chun
 */
pub fn is_tilestring(candidate: &str) -> bool {
    let re = Regex::new(r"^(([0-9]+[mwcpdsb])|([1-7]+[zh]))*$").unwrap();
    return match re.captures(candidate) {
        Some(_) => true,
        None => false,
    };
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

    #[test]
    fn tilestrings_of_complete_hands_are_valid() {
        assert_eq!(is_tilestring("111406p33377z789s"), true);
    }

    #[test]
    fn tilestrings_of_a_few_tiles_are_valid() {
        assert_eq!(is_tilestring("111p37z"), true);
    }

    #[test]
    fn tilestrings_of_random_text_are_invalid() {
        assert_eq!(is_tilestring("helloworld"), false);
    }
}
