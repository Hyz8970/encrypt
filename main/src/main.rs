use core::s_des;
use std::io;
use std::io::Write;
use std::str::FromStr;

fn main() {
    let mut str: String = String::new();
    print!("{}", "enter 8-bit message to encrypt:");
    io::stdout().flush();
    io::stdin().read_line(&mut str).expect("error");
    let mut key: String = String::new();
    print!("{}", "enter a 10-bit key:");
    io::stdout().flush();
    io::stdin().read_line(&mut key).expect("error");
    let str = u8::from_str_radix(str.trim(),2).unwrap_or_default();
    let key = i16::from_str_radix(key.trim(),2).unwrap_or_default();
    let cipher = s_des::encrypt(str, key);
    println!("cipher:{:?}", format!("{:#010b}", &cipher));
    let plaintext = s_des::decrypt(cipher, key);
    println!("plaintext:{:?}", format!("{:#010b}", &plaintext));
}
