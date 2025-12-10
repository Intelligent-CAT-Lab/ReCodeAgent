use textrank::*;

#[test]
fn test_weighting_relation() {
    let rank = create_rank();
    let def = NewAlgorithmDefault();
    let weight_def = def.WeightingRelation(0, 1, &rank);

    assert_eq!(2.0, weight_def);

    let chain = NewAlgorithmChain();
    let weight_chain = chain.WeightingRelation(0, 1, &rank);

    assert_eq!(2.01, weight_chain);

    let weight_chain = chain.WeightingRelation(2, 3, &rank);

    assert_eq!(1.0, weight_chain);
}

#[test]
fn test_weighting_hits() {
    let rank = create_rank();

    let def = NewAlgorithmDefault();
    let weight_def = def.WeightingHits(0, &rank);

    assert_eq!(2.0, weight_def);

    let chain = NewAlgorithmChain();
    let weight_chain = chain.WeightingHits(0, &rank);

    assert_eq!(3.0, weight_chain);

    let weight_chain = chain.WeightingHits(2, &rank);

    assert_eq!(3.0, weight_chain);
}

fn create_rank() -> Rank {
    let mut rank = NewRank();
    rank.AddNewWord("word1".to_string(), -1, 0);
    rank.AddNewWord("word2".to_string(), 0, 0);
    rank.UpdateWord("word1", 1, 0);
    rank.AddNewWord("word3".to_string(), 0, 0);
    rank.AddNewWord("word4".to_string(), 2, 0);

    rank.relation.add_relation(0, 1, 0).unwrap();
    rank.relation.add_relation(1, 0, 0).unwrap();
    rank.relation.add_relation(0, 2, 0).unwrap();
    rank.relation.add_relation(2, 3, 0).unwrap();

    rank.relation.max = 3.0;
    rank.relation.min = 1.0;

    rank
}
