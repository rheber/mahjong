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

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
