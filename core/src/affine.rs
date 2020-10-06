pub fn encrypt(text: &String, key1: i32, key2: i32) -> String {
    let mut result: String = "".to_string();
    for t in text.chars() {
        let temp: i32 = t as i32;
        if temp >= 65 && temp <= 90 {
            let ch = (&key1 * (temp - 65) + &key2) % 26 + 65;
            result.push(char::from(ch as u8));
        } else if temp >= 97 && temp <= 122 {
            let ch = (&key1 * (temp - 97) + &key2) % 26 + 97;
            result.push(char::from(ch as u8));
        } else {
            result.push(t);
        }
    }
    result
}

pub fn decrypt(cipher: &String, key1: i32, key2: i32) -> String {
    let mut result: String = "".to_string();
    let _key1 = inv(&key1, 26);
    println!("inv={}", _key1);
    for t in cipher.chars() {
        let temp: i32 = t as i32;
        if temp >= 65 && temp <= 90 {
            let aa = _key1 * (temp - 65 - key2);
            let mut z = aa % 26 + 65;
            if aa < 0 {
                z += 26;
            }
            result.push(char::from(z as u8));
        } else if temp >= 97 && temp <= 122 {
            let aa = _key1 * (temp - 97 - key2);
            let mut z = aa % 26 + 97;
            if aa < 0 {
                z += 26;
            }
            result.push(char::from(z as u8));
        } else {
            result.push(t);
        }
    }
    result
}

//逆元
fn inv(a: &i32, n: i32) -> i32 {
    let mut arr: Vec<i32> = vec![];
    let mut inv = 0;
    for test in 1..n {
        if 1 == gcd(n, test) {
            arr.push(test);
        }
    }
    for b in arr {
        if 1 == (a * b) % n { inv = b }
    }
    inv
}

//最大公约数
pub fn gcd(v1: i32, v2: i32) -> i32 {
    let mut vv1 = v1;
    let mut vv2 = v2;
    let mut gc = 0;
    let mut div = 0;
    loop {
        div = vv1 % vv2;
        gc = vv2;
        vv1 = vv2;
        vv2 = div;
        if 0 == div { break; }
    }
    gc
}