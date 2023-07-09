/*!
 * Module that defines pais.
 */

use core::hash::Hash;

/**
 * A suit.
 */
#[derive(Clone, Copy, Debug, Hash, PartialEq)]
pub enum Shoku {
    Manzu,
    Pinzu,
    Souzu,
}

/**
 * A wind.
 */
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Kazehai {
    Ton,
    Nan,
    Shaa,
    Pei,
}

/**
 * A dragon.
 */
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Sangenpai {
    Chun,
    Haku,
    Hatsu,
}

/**
 * A number tile.
 */
#[derive(Clone, Copy, Debug)]
pub struct Suupai {
    /**
     * The suit.
     */
    pub shoku: Shoku,

    /**
     * The rank, a number from 1 through 9.
     */
    pub rank: u8,

    /**
     * Whether this tile is a red dora.
     */
    pub akadora: bool,
}

impl Eq for Suupai {

}

// Manually implement Hash to ignore akadora.
impl Hash for Suupai {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.shoku.hash(state);
        self.rank.hash(state);
    }
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
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Jihai {
    Kazehai(Kazehai),
    Sangenpai(Sangenpai),
}

/**
 * A tile.
 */
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Pai {
    Jihai(Jihai),
    Suupai(Suupai),
}
