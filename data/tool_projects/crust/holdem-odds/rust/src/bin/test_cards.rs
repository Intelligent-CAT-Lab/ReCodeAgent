use Holdem_Odds::cards::{Card, Rank, Suit, card_compare, card_swap};
#[test]
fn test_char_to_suit() {
    assert_eq!(Suit::from_char('c'), Suit::Club);
    assert_eq!(Suit::from_char('d'), Suit::Diamond);
    assert_eq!(Suit::from_char('h'), Suit::Heart);
    assert_eq!(Suit::from_char('s'), Suit::Spade);
    let invalid_suits = "abefgijklmnopqrtuvwxyzABCDEFGHIJKLMNOQRSTUVWXYZ0123456789";
    for c in invalid_suits.chars() {
        assert_eq!(Suit::from_char(c), Suit::InvalidSuit);
    }
}
#[test]
fn test_char_to_rank() {
    assert_eq!(Rank::from_char('2'), Rank::Deuce);
    assert_eq!(Rank::from_char('3'), Rank::Trey);
    assert_eq!(Rank::from_char('4'), Rank::Four);
    assert_eq!(Rank::from_char('5'), Rank::Five);
    assert_eq!(Rank::from_char('6'), Rank::Six);
    assert_eq!(Rank::from_char('7'), Rank::Seven);
    assert_eq!(Rank::from_char('8'), Rank::Eight);
    assert_eq!(Rank::from_char('9'), Rank::Nine);
    assert_eq!(Rank::from_char('T'), Rank::Ten);
    assert_eq!(Rank::from_char('J'), Rank::Jack);
    assert_eq!(Rank::from_char('Q'), Rank::Queen);
    assert_eq!(Rank::from_char('K'), Rank::King);
    assert_eq!(Rank::from_char('A'), Rank::Ace);
    let invalid_ranks = "abcdefghijklmnopqrstuvwxyzBCDEFGHILMNOPRSUVWXYZ01";
    for c in invalid_ranks.chars() {
        assert_eq!(Rank::from_char(c), Rank::InvalidRank);
    }
}
#[test]
fn test_card_is_valid() {
    for rank in Rank::Deuce as u8..=Rank::Ace as u8 {
        for suit in Suit::Club as u8..=Suit::Spade as u8 {
            let card = Card::new(unsafe { std::mem::transmute(rank) }, unsafe { std::mem::transmute(suit) });
            assert!(card.is_valid());
        }
    }
    let invalid_cards = [
        Card::new(Rank::InvalidRank, Suit::Spade),
        Card::new(Rank::Ace, Suit::InvalidSuit),
        Card::new(Rank::InvalidRank, Suit::InvalidSuit),
    ];
    for card in &invalid_cards {
        assert!(!card.is_valid());
    }
}
#[test]
fn test_new_card() {
    for rank in Rank::Deuce as u8..=Rank::Ace as u8 {
        for suit in Suit::Club as u8..=Suit::Spade as u8 {
            let card = Card::new(unsafe { std::mem::transmute(rank) }, unsafe { std::mem::transmute(suit) });
            assert!(card.is_valid());
        }
    }
}
#[test]
fn test_new_card_from_chars() {
    for rank in "23456789TJQKA".chars() {
        for suit in "chds".chars() {
            let card = Card::from_chars(rank, suit);
            assert!(card.is_valid());
        }
    }
}
#[test]
fn test_new_card_from_string() {
    for rank in "23456789TJQKA".chars() {
        for suit in "chds".chars() {
            let card = Card::from_string(&format!("{}{}", rank, suit));
            assert!(card.is_valid());
        }
    }
    let invalid_strings = ["", "A", "AAA"];
    for s in &invalid_strings {
        let card = Card::from_string(s);
        assert!(!card.is_valid());
    }
}
#[test]
fn test_card_compare() {
    for rank1 in Rank::Deuce as u8..Rank::Ace as u8 {
        for rank2 in rank1 + 1..=Rank::Ace as u8 {
            let c1 = Card::new(unsafe { std::mem::transmute(rank1) }, Suit::Club);
            let c2 = Card::new(unsafe { std::mem::transmute(rank2) }, Suit::Diamond);
            assert!(card_compare(&c1, &c2) < 0);
            assert!(card_compare(&c2, &c1) > 0);
        }
    }
}
#[test]
fn test_card_swap() {
    let mut c1 = Card::new(Rank::Ace, Suit::Spade);
    let mut c2 = Card::new(Rank::King, Suit::Heart);
    card_swap(&mut c1, &mut c2);
    assert_eq!(c1.rank, Rank::King);
    assert_eq!(c1.suit, Suit::Heart);
    assert_eq!(c2.rank, Rank::Ace);
    assert_eq!(c2.suit, Suit::Spade);
}
pub fn main(){}