// This is a simple encryption algorithm that uses a
// key and a mathematical operation to encrypt and
// decrypt data.

// Choose a mathematical operation to use as the core
// of the encryption algorithm. In this case, we will
// use addition.
fn operation(x: i32, y: i32) -> i32 {
    x + y
}

// Generate a random key to use for encrypting and
// decrypting data. The key should be long and
// random to make it difficult for an attacker to
// guess or reverse engineer.
fn generate_key() -> String {
    let mut rng = rand::thread_rng();
    let key: u64 = rng.gen();
    key.to_string()
}

// Define a function for encrypting data. This
// function takes the data to be encrypted and
// the key as inputs, and returns the encrypted
// data as output.
fn encrypt(data: &str, key: &str) -> Vec<i32> {
    // Convert the data to a string to make it
    // easier to encrypt.
    let data_string = data.to_string();

    // Initialize an empty array to store the
    // encrypted data.
    let mut encrypted_data = Vec::new();

    // Loop through the data string and encrypt
    // each character using the key and the
    // chosen mathematical operation.
    for i in 0..data_string.len() {
        let char_code = data_string.as_bytes()[i] as i32;
        let encrypted_char_code = operation(char_code, key.parse::<i32>().unwrap());
        encrypted_data.push(encrypted_char_code);
    }

    // Return the encrypted data as output.
    encrypted_data
}

// Define a function for decrypting data. This
// function takes the encrypted data and the
// key as inputs, and returns the decrypted
// data as output.
fn decrypt(encrypted_data: Vec<i32>, key: &str) -> String {
    // Initialize an empty string to store the
    // decrypted data.
    let mut decrypted_data = String::new();

    // Loop through the encrypted data and
    // decrypt each character using the key
    // and the chosen mathematical operation.
    for i in 0..encrypted_data.len() {
        let encrypted_char_code = encrypted_data[i];
        let decrypted_char_code = operation(encrypted_char_code, -key.parse::<i32>().unwrap());
        decrypted_data.push(decrypted_char_code as u8 as char);
    }

    // Return the decrypted data as output.
    decrypted_data
}

fn main() {
    // Test the encryption and decryption functions
    // using some sample data.
    let data = "Hello, world!";
    let key = generate_key();
    let encrypted_data = encrypt(data, &key);
    let decrypted_data = decrypt(encrypted_data, &key);

    // Print the original data, the generated key,
    // the encrypted data, and the decrypted data
    // to the console.
    println!("Original data: {}", data);
    println!("Key: {}", key);
    println!("Encrypted data: {:?}", encrypted_data);
    println!("Decrypted data: {}", decrypted_data);
}
