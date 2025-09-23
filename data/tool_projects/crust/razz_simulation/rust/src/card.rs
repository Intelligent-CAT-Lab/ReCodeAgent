pub mod card {
    pub const CLUB_BITS: u32 = 4 << 5;
    pub const ACE_BITS: u32 = 1;
    pub const J_BITS: u32 = 11;
    pub const R8_BITS: u32 = 8;
    pub const R10_BITS: u32 = 10;
    pub const HEART_BITS: u32 = 2 << 5;
    pub const R4_BITS: u32 = 4;
    pub const SPADE_BITS: u32 = 1 << 5;
    pub const Q_BITS: u32 = 12;
    pub const R7_BITS: u32 = 7;
    pub const R5_BITS: u32 = 5;
    pub const K_BITS: u32 = 13;
    pub const R3_BITS: u32 = 3;
    pub const RANK_BITS: u32 = 0x1F;
    pub const R9_BITS: u32 = 9;
    pub const SUIT_BITS: u32 = 0x7 << 5;
    pub const R6_BITS: u32 = 6;
    pub const INVALID_CARD_BITS: u32 = 0;
    pub const DIAMOND_BITS: u32 = 3 << 5;
    pub const R2_BITS: u32 = 2;
    #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
    pub enum CardSuitRank {
        SpadeAce, Spade2, Spade3, Spade4, Spade5, Spade6, Spade7, Spade8,
        Spade9, Spade10, SpadeJ, SpadeQ, SpadeK,
        HeartAce, Heart2, Heart3, Heart4, Heart5, Heart6, Heart7, Heart8,
        Heart9, Heart10, HeartJ, HeartQ, HeartK,
        DiamondAce, Diamond2, Diamond3, Diamond4, Diamond5, Diamond6,
        Diamond7, Diamond8, Diamond9, Diamond10, DiamondJ, DiamondQ, DiamondK,
        ClubAce, Club2, Club3, Club4, Club5, Club6, Club7, Club8, Club9,
        Club10, ClubJ, ClubQ, ClubK,
        CardCount,
        InvalidCard,
    }
    impl CardSuitRank {
        pub fn cardtostr(&self) ->  Option<String> {
            unimplemented!()
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
    pub enum CardRank {
        Ace, R2, R3, R4, R5, R6, R7, R8, R9, R10, J, Q, K,
        RankCount,
        InvalidRank,
    }
    impl CardRank {
        pub fn ranktostr(&self) -> Option<String> {
            unimplemented!();
        }
        pub fn strtorank(str: &str) -> CardRank {
            unimplemented!();
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
    pub enum CardSuit {
        Spade, Heart, Diamond, Club,
        SuitCount,
        InvalidSuit,
    }
    pub struct Card {
        card: u8
    }
    impl Card {
        pub fn write_card(csr: CardSuitRank) -> Self {
            unimplemented!()
        }
        pub fn get_card_suit_rank(&self) -> CardSuitRank {
            unimplemented!();
        }
        pub fn get_card_rank(&self) -> CardRank {
            unimplemented!();
        }
        pub fn get_card_suit(&self) -> CardSuit {
            unimplemented!();
        }
        pub fn create_card(csr: CardSuitRank) -> Option<Self> {
            unimplemented!();
        }
        pub fn strtocard(str: &str) -> Option<Self> {
            unimplemented!();
        }
    }
    pub struct CardCollection {
        prev: Option<Box<CardCollection>>, 
        next: Option<Box<CardCollection>>, 
        c: Option<Card>,
    }
    impl CardCollection {
        pub fn insert_into_collection(self, c: Option<Card>, sorter: CardSorter) -> Self{
            unimplemented!()
        }
        pub fn iterate_collection(&self) -> &Self {
            unimplemented!()
        }
        pub fn append_into_collection(self, new: Self) -> Self {
            unimplemented!()
        }
        pub fn detach_from_collection(&mut self, entry: &Option<Box<CardCollection>>) {
            unimplemented!()
        }
    }
    pub struct CardHand {
        max: u8, 
        len: u8, 
        sorter: CardSorter, 
        cards: CardCollection,
    }
    impl CardHand {
        pub fn create_hand(max: u8, sorter: CardSorter) -> Option<CardHand> {
            unimplemented!();
        }
        pub fn reset_hand(&mut self) {
            unimplemented!();
        }
        pub fn insert_into_hand(&mut self, c: &Option<Card>) {
            unimplemented!();
        }
        pub fn count_cards_in_hand(&self) -> u64 {
            unimplemented!();
        }
        pub fn get_max_of_hand(&self) -> u64 {
            unimplemented!();
        }
        pub fn get_max_rank_of_hand(&self) -> CardRank {
            unimplemented!();
        }
        pub fn iterate_hand(&mut self, itr_fn: CardIterator) {
            unimplemented!();
        }
        pub fn remove_from_hand(&mut self, c: CardSuitRank) {
            unimplemented!();
        }
        pub fn remove_from_hand_under_iter (&mut self, CardCollection: &CardCollection, pos: usize) {
            unimplemented!()
        }
    }
    pub struct CardDeck {
        card_count: u8, 
        cards: [Card; CardSuitRank::CardCount as usize],
    }
    impl CardDeck {
        pub fn is_card_in_deck(&self, c: CardSuitRank) -> i32 {
            unimplemented!();
        }
        pub fn deal_from_deck(&mut self) -> Option<Card> {
            unimplemented!();
        }
        pub fn strip_card_from_deck(&mut self, c: CardSuitRank) {
            unimplemented!();
        }
        pub fn create_shuffled_deck() -> Option<CardDeck> {
            unimplemented!();
        }
    }
    pub type CardSorter = fn(&Option<Card>, &Option<Card>, &Option<Card>) -> i32;
    pub fn sort_card_after(before: &Option<Card>, new: &Option<Card>, after: &Option<Card>) -> i32 {
        unimplemented!();
    }
    pub fn sort_card_by_rank(before: &Option<Card>, new: &Option<Card>, after: &Option<Card>) -> i32 {
        unimplemented!();
    }
    #[derive(Debug, Clone, Copy)]
    pub enum ItrAction {
        Continue,
        Break,
        RemoveAndContinue,
        RemoveAndBreak,
    }
    pub type CardIterator = fn(u64, u64, &Option<Card>) -> ItrAction;
}