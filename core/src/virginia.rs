pub fn encrypt(text: &String, key: String) -> String {
    let mut result = String::new();
    let mut keys: Vec<i32> = vec![];
    for char in key.chars() {
        keys.push(char as i32);
    }
    let key_len = keys.len();
    for (k, char) in text.chars().enumerate() {
        let temp = char as i32;
        if temp >= 65 && temp <= 90 {
            result.push(char::from(((temp - 65 + keys[k % key_len] - 65) % 26 + 65) as u8));
        } else if temp >= 97 && temp <= 122 {
            result.push(char::from(((temp - 97 + keys[k % key_len] - 97) % 26 + 97) as u8));
        } else {
            result.push(char);
        }
    }
    result
}

pub fn decrypt(cipher: &String, key: String) -> String {
    let mut result = String::new();
    let mut keys: Vec<i32> = vec![];
    for char in key.chars() {
        keys.push(char as i32);
    }
    let key_len = keys.len();
    for (k, char) in cipher.chars().enumerate() {
        let temp = char as i32;
        if temp >= 65 && temp <= 90 {
            result.push(char::from(((temp - keys[k % key_len] + 26) % 26 + 65) as u8));
        } else if temp >= 97 && temp <= 122 {
            result.push(char::from(((temp - keys[k % key_len] + 26) % 26 + 97) as u8));
        } else {
            result.push(char);
        }
    }
    result
}