// This is a simple encryption algorithm that uses a
// key and a mathematical operation to encrypt and
// decrypt data.

// Choose a mathematical operation to use as the core
// of the encryption algorithm. In this case, we will
// use addition.
const operation = (x, y) => x + y;

// Generate a random key to use for encrypting and
// decrypting data. The key should be long and
// random to make it difficult for an attacker to
// guess or reverse engineer.
const key = Math.random().toString(36).substring(2);

// Define a function for encrypting data. This
// function takes the data to be encrypted and
// the key as inputs, and returns the encrypted
// data as output.
const encrypt = (data, key) => {
  // Convert the data to a string to make it
  // easier to encrypt.
  const dataString = data.toString();

  // Initialize an empty array to store the
  // encrypted data.
  const encryptedData = [];

  // Loop through the data string and encrypt
  // each character using the key and the
  // chosen mathematical operation.
  for (let i = 0; i < dataString.length; i++) {
    const charCode = dataString.charCodeAt(i);
    const encryptedCharCode = operation(charCode, key);
    encryptedData.push(encryptedCharCode);
  }

  // Return the encrypted data as output.
  return encryptedData;
};

// Define a function for decrypting data. This
// function takes the encrypted data and the
// key as inputs, and returns the decrypted
// data as output.
const decrypt = (encryptedData, key) => {
  // Initialize an empty string to store the
  // decrypted data.
  let decryptedData = "";

  // Loop through the encrypted data and
  // decrypt each character using the key
  // and the chosen mathematical operation.
  for (let i = 0; i < encryptedData.length; i++) {
    const encryptedCharCode = encryptedData[i];
    const decryptedCharCode = operation(encryptedCharCode, -key);
    decryptedData += String.fromCharCode(decryptedCharCode);
  }

  // Return the decrypted data as output.
  return decryptedData;
};

// Test the encryption and decryption functions
// using some sample data.
const data = "Hello, world!";
const encryptedData = encrypt(data, key);
const decryptedData = decrypt(encryptedData, key);

console.log("Original data:", data);
console.log("Encrypted data:", encryptedData);
console.log("Decrypted data:", decryptedData);
