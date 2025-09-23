use Holdem_Odds::bucket::Bucket;
use Holdem_Odds::cards::{Card, Rank, Suit};
#[test]
fn test_bucket() {
    let mut bucket = Bucket::new();
    let c1 = Card::new(Rank::Ace, Suit::Club);
    let c2 = Card::new(Rank::Ace, Suit::Diamond);
    let c3 = Card::new(Rank::Ace, Suit::Heart);
    let c4 = Card::new(Rank::Ace, Suit::Spade);
    assert_eq!(bucket.count, 0, "Bucket should have length 0");
    bucket.add(c1);
    assert_eq!(bucket.count, 1, "Bucket should have length 1");
    assert_eq!(bucket.cards[0], Some(c1), "bucket[0] = Ac");
    bucket.add(c2);
    assert_eq!(bucket.count, 2, "Bucket should have length 2");
    assert_eq!(bucket.cards[0], Some(c1), "bucket[0] = Ac");
    assert_eq!(bucket.cards[1], Some(c2), "bucket[1] = Ad");
    bucket.add(c3);
    assert_eq!(bucket.count, 3, "Bucket should have length 3");
    assert_eq!(bucket.cards[0], Some(c1), "bucket[0] = Ac");
    assert_eq!(bucket.cards[1], Some(c2), "bucket[1] = Ad");
    assert_eq!(bucket.cards[2], Some(c3), "bucket[2] = Ah");
    bucket.add(c4);
    assert_eq!(bucket.count, 4, "Bucket should have length 4");
    assert_eq!(bucket.cards[0], Some(c1), "bucket[0] = Ac");
    assert_eq!(bucket.cards[1], Some(c2), "bucket[1] = Ad");
    assert_eq!(bucket.cards[2], Some(c3), "bucket[2] = Ah");
    assert_eq!(bucket.cards[3], Some(c4), "bucket[3] = As");
    bucket.add(c4);
    assert_eq!(bucket.count, 4, "Bucket should still have length 4");
    assert_eq!(bucket.cards[0], Some(c1), "bucket[0] = Ac");
    assert_eq!(bucket.cards[1], Some(c2), "bucket[1] = Ad");
    assert_eq!(bucket.cards[2], Some(c3), "bucket[2] = Ah");
    assert_eq!(bucket.cards[3], Some(c4), "bucket[3] = As");
}
pub fn main() {
}