pub const RANKS_PER_DECK: usize = 13;
pub const SUITS_PER_DECK: usize = 4;
pub const RANK_CHARS: &str = "23456789TJQKA";
pub const SUIT_CHARS: &str = "cdhs";
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Suit {
    InvalidSuit,
    Club,
    Diamond,
    Heart,
    Spade,
}
#[derive(Debug, PartialEq, Eq, Copy, Clone, PartialOrd, Ord)]
pub enum Rank {
    InvalidRank,
    Deuce,
    Trey,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}
impl Card {
    pub fn new(rank: Rank, suit: Suit) -> Self {
        unimplemented!()
    }
    pub fn from_string(s: &str) -> Card {
        unimplemented!()
    }
    pub fn from_chars(rank: char, suit: char) -> Card {
        unimplemented!()
    }
    pub fn is_valid(&self) -> bool {
        unimplemented!()
    }
    pub fn to_string(&self) -> String {
        unimplemented!()
    }
}
pub fn card_swap(card1: &mut Card, card2: &mut Card) {
    unimplemented!()
}
impl Rank {
    pub fn from_char(c: char) -> Rank {
        unimplemented!()
    }
    pub fn to_char(&self) -> char {
        unimplemented!()
    }
}
impl Suit {
    pub fn from_char(c: char) -> Suit {
        unimplemented!()
    }
    pub fn to_char(&self) -> char {
        unimplemented!()
    }
}
pub fn index_of(c: char, chars: &str) -> Option<usize> {
    unimplemented!()
}
pub fn char_to_rank(c: char) -> Rank {
    unimplemented!()
}
pub fn char_to_suit(c: char) -> Suit {
    unimplemented!()
}
pub fn new_card_from_chars(rank: char, suit: char) -> Card {
    unimplemented!()
}
pub fn card_compare(card1: &Card, card2: &Card) -> i32 {
    unimplemented!()
}
pub fn card_equal(card1: &Card, card2: &Card) -> bool {
    unimplemented!()
}