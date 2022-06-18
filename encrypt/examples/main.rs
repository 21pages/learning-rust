fn main() {
    test_crypt();
    symmetric();
    fixkey_symmetric();
    box_simple();
    box_precomputed();
    sign();
    sign_detach();
}

fn test_crypt() {
    use encrypt::crypt;

    println!("test_crypt");

    let input = vec![1, 2, 3, 4, 5, 6];
    println!("input:{:?}", input);
    if let Ok(encrypted) = crypt(&input, true) {
        println!("encrypted:{:?}", encrypted);
        if let Ok(decrypted) = crypt(&encrypted, false) {
            println!("decrypted:{:?}", decrypted);
        }
    }
}

fn symmetric() {
    use sodiumoxide::crypto::secretbox;

    println!("symmetric");
    let key = secretbox::gen_key();
    let nonce = secretbox::gen_nonce();
    let plaintext = b"some data";
    let ciphertext = secretbox::seal(plaintext, &nonce, &key);
    // let key = secretbox::gen_key();
    // let nonce = secretbox::gen_nonce();
    let their_plaintext = secretbox::open(&ciphertext, &nonce, &key).unwrap();
    println!("before:{}", String::from_utf8_lossy(plaintext));
    println!("after :{}", String::from_utf8_lossy(&their_plaintext));
    assert!(plaintext == &their_plaintext[..]);
}

fn fixkey_symmetric() {
    use sodiumoxide::crypto::secretbox;

    println!("symmetric");
    let key = secretbox::Key([0; secretbox::KEYBYTES]);
    let nonce = secretbox::Nonce([0; secretbox::NONCEBYTES]);
    let plaintext = b"some data";
    let ciphertext = secretbox::seal(plaintext, &nonce, &key);
    println!("{:?}", ciphertext);
    let their_plaintext = secretbox::open(&ciphertext, &nonce, &key).unwrap();
    println!("before:{}", String::from_utf8_lossy(plaintext));
    println!("after :{}", String::from_utf8_lossy(&their_plaintext));
    assert!(plaintext == &their_plaintext[..]);
}

fn box_simple() {
    use sodiumoxide::crypto::box_;

    println!("box_simple");

    let (ourpk, oursk) = box_::gen_keypair();
    // normally theirpk is sent by the other party
    let (theirpk, theirsk) = box_::gen_keypair();
    let nonce = box_::gen_nonce();
    let plaintext = b"some data";
    let ciphertext = box_::seal(plaintext, &nonce, &theirpk, &oursk);
    let their_plaintext = box_::open(&ciphertext, &nonce, &ourpk, &theirsk).unwrap();
    println!("before:{}", String::from_utf8_lossy(plaintext));
    println!("after :{}", String::from_utf8_lossy(&their_plaintext));
    assert!(plaintext == &their_plaintext[..]);
}

fn box_precomputed() {
    use sodiumoxide::crypto::box_;

    println!("box_precomputed");

    let (ourpk, oursk) = box_::gen_keypair();
    let (theirpk, theirsk) = box_::gen_keypair();
    let our_precomputed_key = box_::precompute(&theirpk, &oursk);
    let nonce = box_::gen_nonce();
    let plaintext = b"plaintext";
    let ciphertext = box_::seal_precomputed(plaintext, &nonce, &our_precomputed_key);
    // this will be identical to our_precomputed_key
    let their_precomputed_key = box_::precompute(&ourpk, &theirsk);
    let their_plaintext =
        box_::open_precomputed(&ciphertext, &nonce, &their_precomputed_key).unwrap();
    println!("before:{}", String::from_utf8_lossy(plaintext));
    println!("after :{}", String::from_utf8_lossy(&their_plaintext));
    assert!(plaintext == &their_plaintext[..]);
}

fn sign() {
    use sodiumoxide::crypto::sign;

    println!("sign");

    let (pk, sk) = sign::gen_keypair();
    let data_to_sign = b"some data";
    let signed_data = sign::sign(data_to_sign, &sk);
    let verified_data = sign::verify(&signed_data, &pk).unwrap();

    println!("before:{}", String::from_utf8_lossy(data_to_sign));
    println!("after :{}", String::from_utf8_lossy(&verified_data));

    assert!(data_to_sign == &verified_data[..]);
}

fn sign_detach() {
    use sodiumoxide::crypto::sign;

    println!("sign_detach");

    let (pk, sk) = sign::gen_keypair();
    let data_to_sign = b"some data";
    let signature = sign::sign_detached(data_to_sign, &sk);

    assert!(sign::verify_detached(&signature, data_to_sign, &pk));
}
