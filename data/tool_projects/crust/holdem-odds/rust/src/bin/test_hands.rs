use Holdem_Odds::cards::{Card, Suit, new_card_from_chars, Rank, card_compare};
use Holdem_Odds::hands::{hand_classify, HandType, hand_sort, hand_compare};
use Holdem_Odds::deck::{new_deck, deck_shuffle};
fn new_hand_from_string(hand_str: &str) -> [Card; 5] {
    let mut hand = [
        Card::new(Rank::Deuce, Suit::Club),
        Card::new(Rank::Deuce, Suit::Club),
        Card::new(Rank::Deuce, Suit::Club),
        Card::new(Rank::Deuce, Suit::Club),
        Card::new(Rank::Deuce, Suit::Club),
    ]; 
    let parts: Vec<&str> = hand_str.split_whitespace().collect();
    for (i, part) in parts.iter().enumerate() {
        hand[i] = new_card_from_chars(part.chars().next().unwrap(), part.chars().nth(1).unwrap());
    }
    hand
}
#[test]
fn test_hand_classify_straight_flush() {
    let straight_flushes = [
        "As Ks Qs Js Ts", 
        "Ks Qs Js Ts 9s", 
        "Qs Js Ts 9s 8s", 
        "Js Ts 9s 8s 7s", 
        "Ts 9s 8s 7s 6s",
        "9s 8s 7s 6s 5s", 
        "8s 7s 6s 5s 4s", 
        "7s 6s 5s 4s 3s", 
        "6s 5s 4s 3s 2s",
    ];
    for &hand_str in &straight_flushes {
        let mut hand = new_hand_from_string(hand_str);
        for suit in [Suit::Club, Suit::Diamond, Suit::Heart, Suit::Spade] {
            for card in hand.iter_mut() {
                card.suit = suit;
            }
            assert_eq!(hand_classify(&hand), HandType::StraightFlush, "{} should be a straight flush", hand_str);
        }
    }
}
#[test]
fn test_hand_classify_wheel_flush() {
    let hand = new_hand_from_string("Ac 5c 4c 3c 2c");
    assert_eq!(hand_classify(&hand), HandType::WheelFlush, "A-5-4-3-2 should be a wheel flush");
}
#[test]
fn test_hand_classify_quads() {
    for rank1 in [
        Rank::Deuce, Rank::Trey, Rank::Four, Rank::Five, Rank::Six, Rank::Seven, Rank::Eight,
        Rank::Nine, Rank::Ten, Rank::Jack, Rank::Queen, Rank::King, Rank::Ace,
    ] {
        for rank2 in [
            Rank::Deuce, Rank::Trey, Rank::Four, Rank::Five, Rank::Six, Rank::Seven, Rank::Eight,
            Rank::Nine, Rank::Ten, Rank::Jack, Rank::Queen, Rank::King, Rank::Ace,
        ] {
            if rank1 == rank2 {
                continue;
            }
            for suit in [Suit::Club, Suit::Diamond, Suit::Heart, Suit::Spade] {
                let hand = [
                    Card::new(rank1, Suit::Club),
                    Card::new(rank1, Suit::Diamond),
                    Card::new(rank1, Suit::Heart),
                    Card::new(rank1, Suit::Spade),
                    Card::new(rank2, suit),
                ];
                assert_eq!(hand_classify(&hand), HandType::FourOfAKind, "{:?} should be four of a kind", hand);
            }
        }
    }
}
#[test]
fn test_hand_classify_full_house() {
    for rank1 in [
        Rank::Deuce, Rank::Trey, Rank::Four, Rank::Five, Rank::Six, Rank::Seven, Rank::Eight,
        Rank::Nine, Rank::Ten, Rank::Jack, Rank::Queen, Rank::King, Rank::Ace,
    ] {
        for rank2 in [
            Rank::Deuce, Rank::Trey, Rank::Four, Rank::Five, Rank::Six, Rank::Seven, Rank::Eight,
            Rank::Nine, Rank::Ten, Rank::Jack, Rank::Queen, Rank::King, Rank::Ace,
        ] {
            if rank1 == rank2 {
                continue;
            }
            let hand = [
                Card::new(rank1, Suit::Club),
                Card::new(rank1, Suit::Diamond),
                Card::new(rank1, Suit::Spade),
                Card::new(rank2, Suit::Club),
                Card::new(rank2, Suit::Diamond),
            ];
            assert_eq!(hand_classify(&hand), HandType::FullHouse, "{:?} should be a full house", hand);
        }
    }
}
#[test]
fn test_hand_classify_flush() {
    let flushes = [
        "As 2s 3s 4s 6s", 
        "As 2s 3s 4s 7s", 
        "As 2s 3s 4s 8s", 
        "As 2s 3s 4s 9s",
        "As 2s 3s 4s Ts", 
        "As 2s 3s 4s Js", 
        "As 2s 3s 4s Qs", 
        "As 2s 3s 4s Ks",
    ];
    for &hand_str in &flushes {
        let mut hand : [Card;5]  = hand_str.split_whitespace().map(|part| {
            Card::new(
                Rank::from_char(part.chars().next().unwrap()),
                Suit::Spade, // Assigning default suit, will change dynamically
            )
        }).collect::<Vec<_>>().try_into().expect("Hand should have exactly 5 cards");
        for suit in [Suit::Club, Suit::Diamond, Suit::Heart, Suit::Spade] {
            for card in hand.iter_mut() {
                card.suit = suit;
            }
            assert_eq!(hand_classify(&hand), HandType::Flush, "{:?} should be a flush", hand);
        }
    }
}
#[test]
fn test_hand_classify_straight() {
    let straights = [
        "6s 5c 4s 3h 2d", "7d 6s 5c 4s 3h", "8h 7d 6s 5c 4s", "9s 8h 7d 6s 5c",
        "Tc 9s 8h 7d 6s", "Jd Tc 9s 8h 7d", "Qh Jd Tc 9s 8h", "Ks Qh Jd Tc 9s", "As Ks Qh Jd Tc",
    ];
    for &hand_str in &straights {
        let hand = hand_str.split_whitespace().map(|part| {
            Card::new(
                Rank::from_char(part.chars().next().unwrap()),
                Suit::Spade,
            )
        }).collect::<Vec<_>>().try_into().expect("Hand should have exactly 5 cards");
        assert_eq!(hand_classify(&hand), HandType::Straight, "{:?} should be a straight", hand);
    }
}
#[test]
fn test_hand_classify_wheel() {
    let hand = [
        Card::new(Rank::Ace, Suit::Club),
        Card::new(Rank::Five, Suit::Club),
        Card::new(Rank::Four, Suit::Spade),
        Card::new(Rank::Trey, Suit::Heart),
        Card::new(Rank::Deuce, Suit::Diamond),
    ];
    assert_eq!(hand_classify(&hand), HandType::Wheel, "A-5-4-3-2 should be a wheel");
}
#[test]
fn test_hand_classify_trips() {
    for rank1 in [
        Rank::Deuce, Rank::Trey, Rank::Four, Rank::Five, Rank::Six, Rank::Seven, Rank::Eight,
        Rank::Nine, Rank::Ten, Rank::Jack, Rank::Queen, Rank::King, Rank::Ace,
    ] {
        for rank2 in [
            Rank::Deuce, Rank::Trey, Rank::Four, Rank::Five, Rank::Six, Rank::Seven, Rank::Eight,
            Rank::Nine, Rank::Ten, Rank::Jack, Rank::Queen, Rank::King, Rank::Ace,
        ] {
            if rank1 == rank2 {
                continue;
            }
            for rank3 in [
                Rank::Deuce, Rank::Trey, Rank::Four, Rank::Five, Rank::Six, Rank::Seven, Rank::Eight,
                Rank::Nine, Rank::Ten, Rank::Jack, Rank::Queen, Rank::King, Rank::Ace,
            ] {
                if rank3 == rank1 || rank3 == rank2 {
                    continue;
                }
                let hand = [
                    Card::new(rank1, Suit::Club),
                    Card::new(rank1, Suit::Diamond),
                    Card::new(rank1, Suit::Spade),
                    Card::new(rank2, Suit::Club),
                    Card::new(rank3, Suit::Diamond),
                ];
                assert_eq!(hand_classify(&hand), HandType::ThreeOfAKind, "{:?} should be three of a kind", hand);
            }
        }
    }
}
#[test]
fn test_hand_classify_two_pair() {
    let two_pairs = [
        "As Ac Ks Kc Qs", "As Ac Qs Qc Ks", "As Ac Js Jc Qs", "As Ac Ts Tc Qs", "As Ac 9s 9c Qs",
        "As Ac 8s 8c Qs", "As Ac 7s 7c Qs", "As Ac 6s 6c Qs", "As Ac 5s 5c Qs", "As Ac 4s 4c Qs",
        "As Ac 3s 3c Qs", "As Ac 2s 2c Qs",
    ];
    for &hand_str in &two_pairs {
        let hand = hand_str.split_whitespace().map(|part| {
            Card::new(
                Rank::from_char(part.chars().next().unwrap()),
                Suit::Spade,
            )
        }).collect::<Vec<_>>().try_into().expect("Hand should have exactly 5 cards");
        assert_eq!(hand_classify(&hand), HandType::TwoPair, "{:?} should be a two pair", hand);
    }
}
#[test]
fn test_hand_classify_pair() {
    let pairs = [
        "As Ac Ks 2c Qs", "As Ac Qs 2c Ks", "As Ac Js 2c Qs", "As Ac Ts 2c Qs", "As Ac 9s 2c Qs",
        "As Ac 8s 2c Qs", "As Ac 7s 2c Qs", "As Ac 6s 2c Qs", "As Ac 5s 2c Qs", "As Ac 4s 2c Qs",
        "As Ac 3s 2c Qs", "As Ac 2s 3c Qs",
    ];
    for &hand_str in &pairs {
        let hand = hand_str.split_whitespace().map(|part| {
            Card::new(
                Rank::from_char(part.chars().next().unwrap()),
                Suit::Spade,
            )
        }).collect::<Vec<_>>().try_into().expect("Hand should have exactly 5 cards");
        assert_eq!(hand_classify(&hand), HandType::Pair, "{:?} should be a pair", hand);
    }
}
#[test]
fn test_hand_classify_high_card() {
    let high_cards = [
        "As 3c Ks 2c Qs", "As 3c Qs 2c Ks", "As 3c Js 2c Qs", "As 3c Ts 2c Qs",
        "As 3c 9s 2c Qs", "As 3c 8s 2c Qs", "As 3c 7s 2c Qs", "As 3c 6s 2c Qs",
        "As 3c 5s 2c Qs", "As 3c 4s 2c Qs", "As 4c 3s 2c Qs", "As 3c 2s 4c Qs",
    ];
    for &hand_str in &high_cards {
        let hand = hand_str.split_whitespace().map(|part| {
            Card::new(
                Rank::from_char(part.chars().next().unwrap()),
                Suit::Spade,
            )
        }).collect::<Vec<_>>().try_into().expect("Hand should have exactly 5 cards");
        assert_eq!(hand_classify(&hand), HandType::HighCard, "{:?} should be a high card", hand);
    }
}
#[test]
fn test_hand_sort() {
    let hands= [
        // Royal flush
        "Ts Js Qs Ks As", "As Ks Qs Js Ts",
        "Js Ts Qs Ks As", "As Ks Qs Js Ts",
        "Js Qs Ts Ks As", "As Ks Qs Js Ts",
        "Js Qs Ks Ts As", "As Ks Qs Js Ts",
        "Js Qs Ks As Ts", "As Ks Qs Js Ts",
        // Quads
        "Kc 8c 8d 8h 8s", "8c 8d 8h 8s Kc",
        "8c Kc 8d 8h 8s", "8c 8d 8h 8s Kc",
        "8d 8c Kc 8h 8s", "8c 8d 8h 8s Kc",
        "8c 8d 8h Kc 8s", "8c 8d 8h 8s Kc",
        "8c 8d 8h 8s Kc", "8c 8d 8h 8s Kc",
        // Full house
        "2c 2s Ac Ad Ah", "Ac Ad Ah 2s 2c",
        "2c Ac 2s Ad Ah", "Ac Ad Ah 2s 2c",
        "2c Ac Ad 2s Ah", "Ac Ad Ah 2s 2c",
        "2c Ac Ad Ah 2s", "Ac Ad Ah 2s 2c",
        "Ac 2c 2s Ad Ah", "Ac Ad Ah 2s 2c",
        "Ac 2c Ad 2s Ah", "Ac Ad Ah 2s 2c",
        "Ac 2c Ad Ah 2s", "Ac Ad Ah 2s 2c",
        "Ac Ad 2c 2s Ah", "Ac Ad Ah 2s 2c",
        "Ac Ad 2c Ah 2s", "Ac Ad Ah 2s 2c",
        "Ac Ad Ah 2c 2s", "Ac Ad Ah 2s 2c",
        "Kc Ks 3c 3d 3h", "3c 3d 3h Ks Kc",
        "Kc 3c Ks 3d 3h", "3c 3d 3h Ks Kc",
        "Kc 3c 3d Ks 3h", "3c 3d 3h Ks Kc",
        "Kc 3c 3d 3h Ks", "3c 3d 3h Ks Kc",
        "3c Kc Ks 3d 3h", "3c 3d 3h Ks Kc",
        "3c Kc 3d Ks 3h", "3c 3d 3h Ks Kc",
        "3c Kc 3d 3h Ks", "3c 3d 3h Ks Kc",
        "3c 3d Kc Ks 3h", "3c 3d 3h Ks Kc",
        "3c 3d Kc 3h Ks", "3c 3d 3h Ks Kc",
        "3c 3d 3h Kc Ks", "3c 3d 3h Ks Kc",
        // Trips
        "2c 3s Ac Ad Ah", "Ac Ad Ah 3s 2c",
        "2c Ac 3s Ad Ah", "Ac Ad Ah 3s 2c",
        "2c Ac Ad 3s Ah", "Ac Ad Ah 3s 2c",
        "2c Ac Ad Ah 3s", "Ac Ad Ah 3s 2c",
        "Ac 2c 3s Ad Ah", "Ac Ad Ah 3s 2c",
        "Ac 2c Ad 3s Ah", "Ac Ad Ah 3s 2c",
        "Ac 2c Ad Ah 3s", "Ac Ad Ah 3s 2c",
        "Ac Ad 2c 3s Ah", "Ac Ad Ah 3s 2c",
        "Ac Ad 2c Ah 3s", "Ac Ad Ah 3s 2c",
        "Ac Ad Ah 2c 3s", "Ac Ad Ah 3s 2c",
        "3c 2s Ac Ad Ah", "Ac Ad Ah 3s 2c",
        "3c Ac 2s Ad Ah", "Ac Ad Ah 3s 2c",
        "3c Ac Ad 2s Ah", "Ac Ad Ah 3s 2c",
        "3c Ac Ad Ah 2s", "Ac Ad Ah 3s 2c",
        "Ac 3c 2s Ad Ah", "Ac Ad Ah 3s 2c",
        "Ac 3c Ad 2s Ah", "Ac Ad Ah 3s 2c",
        "Ac 3c Ad Ah 2s", "Ac Ad Ah 3s 2c",
        "Ac Ad 3c 2s Ah", "Ac Ad Ah 3s 2c",
        "Ac Ad 3c Ah 2s", "Ac Ad Ah 3s 2c",
        "Ac Ad Ah 3c 2s", "Ac Ad Ah 3s 2c",
    ];
    for pair in hands.chunks_exact(2) {
        let mut hand1: [Card; 5] = pair[0].split_whitespace().map(|part| {
            Card::new(
                Rank::from_char(part.chars().next().unwrap()),
                Suit::Spade,
            )
        }).collect::<Vec<_>>().try_into().expect("Hand should have exactly 5 cards");
        let hand2:[Card; 5] = pair[1].split_whitespace().map(|part| {
            Card::new(
                Rank::from_char(part.chars().next().unwrap()),
                Suit::Spade,
            )
        }).collect::<Vec<_>>().try_into().expect("Hand should have exactly 5 cards");
        hand_sort(&mut hand1);
        assert!(hand1.iter().zip(hand2.iter()).all(|(a, b)| card_compare(a, b) == 0), "Sorted hand {:?} should match {:?}", hand1, hand2);
    }
}
#[test]
fn test_hand_compare() {
    let hands = [
        // Royal flush
        "Ts Js Qs Ks As", "9s Ts Js Qs Ks", // vs straight flush
        "Ts Js Qs Ks As", "As Ad Ks Ah Ac", // vs quads
        "Ts Js Qs Ks As", "As Ad Ks Kc Ah", // vs full house
        "Ts Js Qs Ks As", "As 3s 5s 7s 8s", // vs flush
        "Ts Js Qs Ks As", "3s 4d 5h 6c 7s", // vs straight
        "Ts Js Qs Ks As", "As 2d Ac 3h Ah", // vs trips
        "Ts Js Qs Ks As", "As Ks Ad Kd 3h", // vs two pair
        "Ts Js Qs Ks As", "As 2d 3h 4c Ac", // vs pair
        "Ts Js Qs Ks As", "As 9d 8h 3s 2s", // vs high card
        // All straight flushes are stronger than the wheel flush
        "Ac Kc Qc Jc Tc", "5c 4c 3c 2c Ac",
        "Kc Qc Jc Tc 9c", "5c 4c 3c 2c Ac",
        "Qc Jc Tc 9c 8c", "5c 4c 3c 2c Ac",
        "Jc Tc 9c 8c 7c", "5c 4c 3c 2c Ac",
        "Tc 9c 8c 7c 6c", "5c 4c 3c 2c Ac",
        "9c 8c 7c 6c 5c", "5c 4c 3c 2c Ac",
        "8c 7c 6c 5c 4c", "5c 4c 3c 2c Ac",
        "7c 6c 5c 4c 3c", "5c 4c 3c 2c Ac",
        "6c 5c 4c 3c 2c", "5c 4c 3c 2c Ac",
        // Quads
        "Ac Ad Ah As Ks", "Ac Ad Ah As Qs", // vs quads, lesser kicker
        "Ac Ad Ah As Ks", "Kc Kd Kh Ks As", // vs quads, lesser quads
        "Ac Ad Ah As Ks", "As Ad Ks Kc Ah", // vs full house
        "Ac Ad Ah As Ks", "As 3s 5s 7s 8s", // vs flush
        "Ac Ad Ah As Ks", "3s 4d 5h 6c 7s", // vs straight
        "Ac Ad Ah As Ks", "As 2d Ac 3h Ah", // vs trips
        "Ac Ad Ah As Ks", "As Ks Ad Kd 3h", // vs two pair
        "Ac Ad Ah As Ks", "As 2d 3h 4c Ac", // vs pair
        "Ac Ad Ah As Ks", "As 9d 8h 3s 2s", // vs high card
        // Full house
        "Ac Ad Ah Kd Ks", "Ac Ad Ah Qd Qs", // vs full house, lesser twos
        "Ac Ad Ah Kd Ks", "Qc Qd Qh Ac Ad", // vs full house, lesser threes
        "Ac Ad Ah Kd Ks", "As 3s 5s 7s 8s", // vs flush
        "Ac Ad Ah Kd Ks", "3s 4d 5h 6c 7s", // vs straight
        "Ac Ad Ah Kd Ks", "As 2d Ac 3h Ah", // vs trips
        "Ac Ad Ah Kd Ks", "As Ks Ad Kd 3h", // vs two pair
        "Ac Ad Ah Kd Ks", "As 2d 3h 4c Ac", // vs pair
        "Ac Ad Ah Kd Ks", "As 9d 8h 3s 2s", // vs high card
        // Flush
        "Ac Qc Tc 8c 6c", "Ac Qc Tc 8c 5c", // vs lesser flush 1
        "Ac Qc Tc 8c 6c", "Ac Qc Tc 7c 6c", // vs lesser flush 2
        "Ac Qc Tc 8c 6c", "Ac Qc 9c 8c 6c", // vs lesser flush 3
        "Ac Qc Tc 8c 6c", "Ac Jc Tc 8c 6c", // vs lesser flush 4
        "Ac Qc Tc 8c 6c", "Kc Qc Tc 8c 6c", // vs lesser flush 5
        "Ac Qc Tc 8c 6c", "3s 4d 5h 6c 7s", // vs straight
        "Ac Qc Tc 8c 6c", "As 2d Ac 3h Ah", // vs trips
        "Ac Qc Tc 8c 6c", "As Ks Ad Kd 3h", // vs two pair
        "Ac Qc Tc 8c 6c", "As 2d 3h 4c Ac", // vs pair
        "Ac Qc Tc 8c 6c", "As 9d 8h 3s 2s", // vs high card
        // Straight
        "Ac Kd Qh Js Tc", "Kd Qh Js Tc 9d", // vs lesser straight
        "Ac Kd Qh Js Tc", "As 2d Ac 3h Ah", // vs trips
        "Ac Kd Qh Js Tc", "As Ks Ad Kd 3h", // vs two pair
        "Ac Kd Qh Js Tc", "As 2d 3h 4c Ac", // vs pair
        "Ac Kd Qh Js Tc", "As 9d 8h 3s 2s", // vs high card
        // All straights are stronger than a wheel
        "Ac Kd Qh Js Tc", "5d 4h 3s 2c Ac",
        "Kd Qh Js Tc 9d", "5d 4h 3s 2c Ac",
        "Qh Js Tc 9d 8h", "5d 4h 3s 2c Ac",
        "Js Tc 9d 8h 7s", "5d 4h 3s 2c Ac",
        "Tc 9d 8h 7s 6s", "5d 4h 3s 2c Ac",
        "9d 8h 7s 6s 5c", "5d 4h 3s 2c Ac",
        "8h 7s 6s 5c 4d", "5d 4h 3s 2c Ac",
        "7s 6s 5c 4d 3h", "5d 4h 3s 2c Ac",
        "6s 5c 4d 3h 2s", "5d 4h 3s 2c Ac",
        // Trips
        "As Ac Ah Jh 8d", "As Ac Ah Jh 7d", // vs trips, lesser kicker 1
        "As Ac Ah Jh 8d", "As Ac Ah Th 8d", // vs trips, lesser kicker 2
        "As Ac Ah Jh 8d", "Ks Kc Kh Ah Qd", // vs trips, lesser three cards
        "As Ac Ah Jh 8d", "As Ks Ad Kd 3h", // vs two pair
        "As Ac Ah Jh 8d", "As 2d 3h 4c Ac", // vs pair
        "As Ac Ah Jh 8d", "As 9d 8h 3s 2s", // vs high card
        // Two pair
        "Ac Ad Jh Js 8s", "Ac Ad Jh Js 7s", // vs two pair, lesser kicker
        "Ac Ad Jh Js 8s", "Ac Ad Th Ts Qs", // vs two pair, lesser low pair
        "Ac Ad Jh Js 8s", "Kc Kd Jh Js Qs", // vs two pair, lesser high pair
        "Ac Ad Jh Js 8s", "As 2d 3h 4c Ac", // vs pair
        "Ac Ad Jh Js 8s", "As 9d 8h 3s 2s", // vs high card
        // Pair
        "Ac Ad Qh 8s 4c", "Ac Ad Qh 8s 3c", // vs pair, lesser kicker 1
        "Ac Ad Qh 8s 4c", "Ac Ad Qh 7s 4c", // vs pair, lesser kicker 2
        "Ac Ad Qh 8s 4c", "Ac Ad Jh 8s 4c", // vs pair, lesser kicker 3
        "Ac Ad Qh 8s 4c", "Kc Kd Qh 8s 4c", // vs pair, lesser pair
        "Ac Ad Qh 8s 4c", "As 9d 8h 3s 2s", // vs high card
        // High card
        "Ac Qd Th 8s 6c", "Ac Qd Th 8s 5c", // vs high card, lesser kicker 1
        "Ac Qd Th 8s 6c", "Ac Qd Th 7s 6c", // vs high card, lesser kicker 2
        "Ac Qd Th 8s 6c", "Ac Qd 9h 8s 6c", // vs high card, lesser kicker 3
        "Ac Qd Th 8s 6c", "Ac Jd Th 8s 6c", // vs high card, lesser kicker 4
        "Ac Qd Th 8s 6c", "Kc Qd Th 8s 6c", // vs high card, lesser kicker 5
    ];
    for pair in hands.chunks_exact(2) {
        let mut hand1 = pair[0].split_whitespace().map(|part| {
            Card::new(
                Rank::from_char(part.chars().next().unwrap()),
                Suit::Spade,
            )
        }).collect::<Vec<_>>().try_into().expect("Hand should have exactly 5 cards");
        let mut hand2 = pair[1].split_whitespace().map(|part| {
            Card::new(
                Rank::from_char(part.chars().next().unwrap()),
                Suit::Spade,
            )
        }).collect::<Vec<_>>().try_into().expect("Hand should have exactly 5 cards");
        hand_sort(&mut hand1);
        hand_sort(&mut hand2);
        assert!(hand_compare(&hand1, &hand2) > 0, "{:?} should be greater than {:?}", hand1, hand2);
        assert!(hand_compare(&hand2, &hand1) < 0, "{:?} should be less than {:?}", hand2, hand1);
        assert!(hand_compare(&hand1, &hand1) == 0, "{:?} should be equal to itself", hand1);
        assert!(hand_compare(&hand2, &hand2) == 0, "{:?} should be equal to itself", hand2);
    }
}
fn main(){}