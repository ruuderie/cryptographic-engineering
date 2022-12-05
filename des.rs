use des::block_cipher_trait::generic_array::GenericArray;
use des::block_cipher_trait::BlockCipher;
use des::Des;

// Define a function that takes a key and a plaintext
// as input and returns the encrypted plaintext and
// the decrypted plaintext after applying the
// complementation property.
fn test_complementation(key: &str, plaintext: &str) -> (Vec<u8>, Vec<u8>) {
    // Convert the key and plaintext to arrays of
    // bytes to make them compatible with the DES
    // encryption and decryption functions.
    let key_bytes = key.as_bytes();
    let key_array = GenericArray::from_slice(key_bytes);
    let plaintext_bytes = plaintext.as_bytes();
    let plaintext_array = GenericArray::from_slice(plaintext_bytes);

    // Initialize a DES cipher instance with the
    // given key.
    let cipher = Des::new(key_array);

    // Encrypt the plaintext using the DES cipher
    // and the given key.
    let encrypted_text = cipher.encrypt(plaintext_array).to_vec();

    // Decrypt the encrypted text using the DES
    // cipher and the complementation of the key.
    let complemented_key_array = key_array
        .iter()
        .map(|x| !x)
        .collect::<Vec<u8>>();
    let complemented_key_array = GenericArray::from_slice(&complemented_key_array);
    let cipher = Des::new(complemented_key_array);
    let decrypted_text = cipher.decrypt(encrypted_text).to_vec();

    // Return the encrypted text and the decrypted
    // text after applying the complementation
    // property.
    (encrypted_text, decrypted_text)
}

fn main() {
    // Test the complementation property using
    // a sample key and plaintext.
    let key = "abcdefgh";
    let plaintext = "hello world";
    let (encrypted_text, decrypted_text) = test_complementation(key, plaintext);

    // Print the original plaintext, the encrypted
    // text, and the decrypted text to the console.
    println!("Original plaintext: {}", plaintext);
    println!("Encrypted text: {:?}", encrypted_text);
    println!("Decrypted text: {:?}", decrypted_text);

    // Check if the decrypted text is equal to the
    // original plaintext to verify that the
    // complementation property holds for this
    // key and plaintext.
    if decrypted_text == plaintext.as_bytes() {
        println!("The complementation property holds for this key and plaintext.");
    } else {
        println!("The complementation property does not hold for this key and plaintext.");
    }
}
