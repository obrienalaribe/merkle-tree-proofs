#![allow(unused)]

use std::str::SplitWhitespace;
use bip32::{Mnemonic, Prefix, XPrv};
use blake2::{Blake2b, Blake2b512, Blake2s256, Digest};


/// Your self-generated, 24 word seed phrase:
pub fn mnemonic() -> Vec<String>{
    let mnemonic= "sweet curious poet error name casual crouch onion artefact huge barely butter later lecture potato
     live hurry announce vanish begin video vast defy appear".split_whitespace()
        .map(|s| s.to_string())
        .collect();
    mnemonic

}

/// Challenge question TBA!
/// Task is to return the flag from this function.
/// (DO NOT change the function signature!)
pub fn flag_a() -> String  {
    let mut hasher = Blake2s256::new();
    hasher.update(mnemonic().join(" ").as_bytes());
    let hash_value = hasher.finalize();
    hex::encode(hash_value)
}

/// Challenge question TBA!
/// Task is to return the flag from this function.
/// (DO NOT change the function signature!)
pub fn flag_b() -> String {
    let phrase = mnemonic().join(" ");
    let mnemonic = Mnemonic::new(phrase, Default::default()).unwrap();
    let seed = mnemonic.to_seed("");
    let root_xprv = XPrv::new(seed).unwrap();
    root_xprv.public_key().to_string(Prefix::XPUB)
}

/// Challenge question TBA!
/// Task is to return the flag from this function.
/// (DO NOT change the function signature!)
pub fn flag_c() {
    todo!()
}

/// Challenge question TBA!
/// Task is to return the flag from this function.
/// (DO NOT change the function signature!)
pub fn flag_d() {
    todo!()
}

/// Challenge question TBA!
/// Task is to return the flag from this function.
/// (DO NOT change the function signature!)
pub fn flag_e() {
    todo!()
}

/// Challenge question TBA!
/// Task is to return the flag from this function.
/// (DO NOT change the function signature!)
pub fn flag_f() {
    todo!()
}

/// Challenge question TBA!
/// Task is to return the flag from this function.
/// (DO NOT change the function signature!)
pub fn flag_g() {
    todo!()
}
