//! This is minimal Merkle tree implementation with proof checking
#![allow(unused)] // silence unused warnings while exploring (to comment out)

use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};
use crate::merkle::SiblingNode::Right;

/// We'll use Rust's built-in hashing which returns a u64 type.
/// This alias just helps us understand when we're treating the number as a hash
type HashValue = u64;

/// Helper function that makes the hashing interface easier to understand.
pub fn hash<T: Hash>(t: String) -> HashValue {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

/// Given a vector of data blocks this function adds padding blocks to the end
/// until the length is a power of two which is needed for Merkle trees.
pub fn pad_base_layer(blocks: &mut Vec<&str>) {
    let n_extra_pads = (blocks.len().next_power_of_two() - blocks.len()) as u64;
    for n in 1..=n_extra_pads {
        blocks.push("");
    }
}
/// Helper function to combine two hashes and compute the hash of the combination.
/// This will be useful when building the intermediate nodes in the Merkle tree.
/// Our implementation will convert the hashes to strings, concatenate the strings,
/// and then hash the strings.
pub fn concatenate_hash_values(left: HashValue, right: HashValue) -> HashValue {
    hash::<String>(format!("{}{}", left, right))
}

pub fn iterate_and_concatenate(hashes: Vec<HashValue> ) -> HashValue {

    if hashes.len() == 1{
        return hashes[0]
    }
    let mut parent_hashes = Vec::new();
    for i in (0..hashes.len()).step_by(2)  {
        let left = hashes[i];
        let right = hashes[i+1];

        let parent_hash = concatenate_hash_values(left, right);
        parent_hashes.push(parent_hash);
    }
    iterate_and_concatenate(parent_hashes)
}


/// Calculates the Merkle root of a sentence. We consider each word in the sentence to
/// be one block. Words are separated by one or more spaces.
///
/// Example:
/// Sentence: "Trust me, bro!"
/// "Trust", "me," "bro!"
/// Notice that the punctuation like the comma and exclamation point are included in the words
/// but the spaces are not.
pub fn calculate_merkle_root(sentence: &str) -> HashValue {
    let mut words_in_sentence: Vec<&str> = sentence.split(" ").collect();
    pad_base_layer(&mut words_in_sentence);

    // Hash all elements in leaf
    let hashed_leaves: Vec<HashValue> = words_in_sentence.into_iter().map(|x: &str| hash::<String>(String::from(x))).collect();
    iterate_and_concatenate(hashed_leaves)
}

/// A representation of a sibling node along the Merkle path from the data
/// to the root. It is necessary to specify which side the sibling is on
/// so that the hash values can be combined in the same order.
#[derive(Debug, PartialEq)]
pub enum SiblingNode {
    Left(HashValue),
    Right(HashValue),
}
/// Generates a Merkle proof that one particular word is contained
/// in the given sentence. You provide the sentence and the index of the word
/// which you want a proof.
///
/// Panics if the index is beyond the length of the word.
///
/// Example: I want to prove that the word "Trust" is in the sentence "Trust me, bro!"
/// So I call generate_proof("Trust me, bro!", 0)
/// And I get back the merkle root and list of intermediate nodes from which the
/// root can be reconstructed.
pub fn generate_proof(sentence: &str, index: usize) -> (HashValue, Vec<SiblingNode>) {
    let mut words_in_sentence: Vec<&str> = sentence.split(" ").collect();

    pad_base_layer(&mut words_in_sentence);

    let mut hashed_leaves: Vec<HashValue> = words_in_sentence.into_iter().map(|x: &str| hash::<String>(String::from(x))).collect();

    // start here
    let mut index = 0;
    let mut neighbours: Vec<SiblingNode> = Vec::new();

    while hashed_leaves.len() > 1 {
        let mut set_of_nodes: Vec<HashValue> = Vec::new();
        if index % 2 == 0 {
            neighbours.push(SiblingNode::Right(hashed_leaves[index + 1]));
        } else {
            neighbours.push(SiblingNode::Left(hashed_leaves[index-1]))
        }

        index = index / 2;

        for i in (0..hashed_leaves.len() / 2) {
            let left_hash = hashed_leaves[i * 2];
            let right_hash = hashed_leaves[(i * 2) + 1];

            let concantenated_hash = concatenate_hash_values(left_hash, right_hash);
            set_of_nodes.push(concantenated_hash);
        }
        hashed_leaves = set_of_nodes;
    }
    (hashed_leaves[0], neighbours)
}

/// Checks whether the given word is contained in a sentence, without knowing the whole sentence.
/// Rather we only know the merkle root of the sentence and a proof.
pub fn validate_proof(root: &HashValue, word: &str, proof: Vec<SiblingNode>) -> bool {
    let mut hashed_word: HashValue = hash::<String>(String::from(word));
    for node in proof {
        match node {
            SiblingNode::Left(hash_value) => {
                hashed_word = concatenate_hash_values(hash_value, hashed_word)
            }
            Right(hash_value) => {
                hashed_word = concatenate_hash_values(hashed_word, hash_value)
            }
        }
    }

    *root == hashed_word
}