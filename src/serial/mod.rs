/*!
 * Module for serialization and deserialization functions.
 */

use crate::rules::*;

use regex::Regex;

/**
 * Whether the given candidate is a valid tilestring e.g. 111406p33377z789s.
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

/**
 * Converts a tilestring to tiles.
 */
pub fn tilestring_to_pais(tilestring: &str) -> Option<Vec<Pai>> {
    if !is_tilestring(tilestring) {
        return None;
    }

    let re = Regex::new(r"([0-9]+[mwcpdsb])|([1-7]+[zh])").unwrap();
    let clumps = re.find_iter(tilestring);
    let clump_pair_lists = clumps.map(|clump| {
        let mut clump_string = clump.as_str().to_owned();
        let letter = clump_string.pop().unwrap();
        let digits = clump_string.chars();
        let pairs = digits.map(move |digit| (digit, letter));
        pairs.collect::<Vec<(char, char)>>()
    });
    let clump_pairs = clump_pair_lists.flatten();

    fn clump_pair_to_pai(dl: (char, char)) -> Option<Pai> {
        let (digit, letter) = dl; 
        if letter == 'z' || letter == 'h' {
            match digit {
                '1' => return Some(Pai::Jihai(Jihai::Kazehai(Kazehai::Ton))),
                '2' => return Some(Pai::Jihai(Jihai::Kazehai(Kazehai::Nan))),
                '3' => return Some(Pai::Jihai(Jihai::Kazehai(Kazehai::Shaa))),
                '4' => return Some(Pai::Jihai(Jihai::Kazehai(Kazehai::Pei))),
                '5' => return Some(Pai::Jihai(Jihai::Sangenpai(Sangenpai::Chun))),
                '6' => return Some(Pai::Jihai(Jihai::Sangenpai(Sangenpai::Haku))),
                '7' => return Some(Pai::Jihai(Jihai::Sangenpai(Sangenpai::Hatsu))),
                _ => return None,
            }
        } else {
            let shoku = match letter {
                'm' | 'w' | 'c' => Shoku::Manzu,
                'p' | 'd' => Shoku::Pinzu,
                's' | 'b' => Shoku::Souzu,
                _ => return None,
            };
            return Some(Pai::Suupai(Suupai {
                shoku,
                rank: if digit == '0' { 5 } else { i8::try_from(digit.to_digit(10).unwrap()).unwrap() },
                akadora: digit == '0',
            }));
        }
    }

    let pais: Vec<Pai> = clump_pairs.map(|dl| clump_pair_to_pai(dl).unwrap()).collect();
    return Some(pais);
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

    #[test]
    fn converts_tilestrings_to_pais() {
        let pais = tilestring_to_pais("111406p33377z789s").unwrap();
        assert_eq!(pais.len(), 14);
        assert_eq!(pais[0], Pai::Suupai(Suupai { shoku: Shoku::Pinzu, rank: 1, akadora: false }));
        assert_eq!(pais[4], Pai::Suupai(Suupai { shoku: Shoku::Pinzu, rank: 5, akadora: true }));
        assert_eq!(pais[10], Pai::Jihai(Jihai::Sangenpai(Sangenpai::Hatsu)));
        assert_eq!(pais[13], Pai::Suupai(Suupai { shoku: Shoku::Souzu, rank: 9, akadora: false }));
    }
}
