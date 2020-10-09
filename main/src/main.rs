use core::virginia;
use std::io;
use std::io::Write;

fn main() {
    let mut str: String = String::new();
    print!("{}", "enter a message to encrypt:");
    io::stdout().flush();
    io::stdin().read_line(&mut str).expect("error");
    let mut key: String = String::new();
    print!("{}", "enter a key:");
    io::stdout().flush();
    io::stdin().read_line(&mut key).expect("error");

    let str=str.trim().to_string();
    let key=key.trim().to_string();

    let cipher = virginia::encrypt(&str, key.clone());
    println!("cipher : {}", &cipher);
    let plaintext = virginia::decrypt(&cipher, key);
    println!("plaintext : {}", &plaintext);
}
