use regex::Regex;

/**
 * A suit.
 */
pub enum Shoku {
    Manzu,
    Pinzu,
    Souzu,
}

/**
 * A wind.
 */
pub enum Kazehai {
    Ton,
    Nan,
    Shaa,
    Pei,
}

/**
 * A dragon.
 */
pub enum Sangenpai {
    Chun,
    Haku,
    Hatsu,
}

/**
 * A number tile.
 */
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
pub enum Jihai {
    Kazehai(Kazehai),
    Sangenpai(Sangenpai),
}

/**
 * A tile.
 */
pub enum Pai {
    Jihai(Jihai),
    Suupai(Suupai),
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
