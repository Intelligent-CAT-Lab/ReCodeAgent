use crate::cards::Card;
pub struct Bucket {
    pub cards: [Option<Card>; 4],
    pub count: usize,
}
impl Bucket {
    pub fn new() -> Self {
        unimplemented!()
    }
    pub fn add(&mut self, card: Card) {
        unimplemented!()
    }
}