use Holdem_Odds::cards::{Card, Rank, Suit, card_compare};
use Holdem_Odds::deck::{new_deck, deck_shuffle};
use rand::thread_rng;
#[test]
fn test_deck_shuffle() {
    let mut deck1 = new_deck();
    let mut deck2 = new_deck();
    deck_shuffle(&mut deck2);
    for card in &deck2 {
        assert!(card.is_valid(), "Deck contains an invalid card");
    }
    let decks_differ = deck1.iter().zip(&deck2).any(|(c1, c2)| card_compare(c1, c2) == 0);
    assert!(decks_differ, "Deck was not shuffled");
}
#[test]
fn test_new_deck() {
    let deck = new_deck();
    let mut expected_deck = Vec::new();
    for suit in [Suit::Club, Suit::Diamond, Suit::Heart, Suit::Spade] {
        for rank in [Rank::Deuce, Rank::Trey, Rank::Four, Rank::Five, Rank::Six, Rank::Seven, Rank::Eight,
                     Rank::Nine, Rank::Ten, Rank::Jack, Rank::Queen, Rank::King, Rank::Ace] {
            expected_deck.push(Card::new(rank, suit));
        }
    }
    assert_eq!(deck, expected_deck, "Deck is not properly initialized");
}
fn main(){}