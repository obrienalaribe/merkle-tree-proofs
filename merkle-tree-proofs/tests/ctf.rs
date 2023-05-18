use blake2::Blake2s256;
use pba_assignment::ctf::*;

#[test]
fn seed_is_24_words() {
    assert_eq!(mnemonic().len(), 24);
}

#[test]
fn test_hashed_hex_of_seed_phrase(){
    assert_eq!(flag_a(), "15ee3ca6e49f56aa2b743ff95cc1721d512f6683548afc8114217564af980dac")
}

#[test]
fn test_flag_b(){
    assert_eq!(flag_b(), "xpub661MyMwAqRbcEpEmA1w7ih8nNA7iyLL1nPWe6rTbDaKoP4rCGpSMbZhKHsLZ5DDjr2LXSbif7ARp7bKPtRxRMFEKtKJQz9TdGSTRLwZWG9e")
}