/*!
 * Module that defines pais.
 */

/**
 * A suit.
 */
#[derive(Debug, PartialEq)]
pub enum Shoku {
    Manzu,
    Pinzu,
    Souzu,
}

/**
 * A wind.
 */
#[derive(Debug, PartialEq)]
pub enum Kazehai {
    Ton,
    Nan,
    Shaa,
    Pei,
}

/**
 * A dragon.
 */
#[derive(Debug, PartialEq)]
pub enum Sangenpai {
    Chun,
    Haku,
    Hatsu,
}

/**
 * A number tile.
 */
#[derive(Debug)]
pub struct Suupai {
    /**
     * The suit.
     */
    pub shoku: Shoku,

    /**
     * The rank, a number from 1 through 9.
     */
    pub rank: i8,

    /**
     * Whether this tile is a red dora.
     */
    pub akadora: bool,
}

// Manually implement PartialEq to ignore akadora.
impl PartialEq for Suupai {
    fn eq(&self, other: &Self) -> bool {
        self.shoku == other.shoku && self.rank == other.rank
    }
}

/**
 * An honor tile.
 */
#[derive(Debug, PartialEq)]
pub enum Jihai {
    Kazehai(Kazehai),
    Sangenpai(Sangenpai),
}

/**
 * A tile.
 */
#[derive(Debug, PartialEq)]
pub enum Pai {
    Jihai(Jihai),
    Suupai(Suupai),
}
