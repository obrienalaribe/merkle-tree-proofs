use pba_assignment::merkle::*;
use pba_assignment::merkle::SiblingNode::Right;


#[test]
fn test_pad_base_layer_0() {
    let mut blocks = Vec::new();
    pad_base_layer(&mut blocks);
    assert_eq!(1, blocks.len());
}

#[test]
fn test_pad_base_layer_1() {
    let mut blocks = vec!["this"];
    pad_base_layer(&mut blocks);
    assert_eq!(1, blocks.len());
}

#[test]
fn test_pad_base_layer_2() {
    let mut blocks = vec!["this", "is", "a", "secret", "text", "that"];
    pad_base_layer(&mut blocks);
    assert_eq!(8, blocks.len());
}


#[test]
fn test_pad_base_layer_3() {
    let mut blocks = vec!["this", "is", "a", "secret", "text", "that", "is", "safe", "good"];
    pad_base_layer(&mut blocks);
    assert_eq!(16, blocks.len());
}

#[test]
fn test_concatenate_hash_values() {
    type HashValue = u64;
    let left_hash: HashValue = 123;
    let right_hash: HashValue = 456;
    let result: HashValue = 12486799259383083265;

    assert_eq!(result, concatenate_hash_values(left_hash, right_hash));
}


#[test]
fn test_concatenate_hash_values_with_max() {
    type HashValue = u64;
    let left_hash: HashValue = u64::MAX;
    let right_hash: HashValue = u64::MAX;
    let result: HashValue = 2128654721519831227;

    assert_eq!(result, concatenate_hash_values(left_hash, right_hash));
}

#[test]
fn test_merkle_root() {
    type HashValue = u64;
    let root_hash: HashValue = 9380130514907307572;
    assert_eq!(root_hash, calculate_merkle_root("Trust me, bro!"));
}

#[test]
fn test_generate_proof() {
    type HashValue = u64;
    let expected = (9380130514907307572 as HashValue, vec![Right(2379988093487823985), Right(9403510424442337973)]);
    let actual = generate_proof("Trust me, bro!", 0);
    assert_eq!(expected, actual);
}

#[test]
fn test_validate_proof() {
    type HashValue = u64;
    let root: HashValue = 9380130514907307572;
    assert!(validate_proof(&root, "Trust", vec![Right(2379988093487823985), Right(9403510424442337973)]))
}