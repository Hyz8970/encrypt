use core::affine;
use core::hill;
use std::io;
use std::io::Write;

fn main() {
    //仿射密码
    // let mut str:String=String::new();
    // let mut key1:String=String::new();
    // let mut key2:String=String::new();
    // print!("{}","enter a multiplicative key:");
    // io::stdout().flush();
    // io::stdin().read_line(&mut key1).expect("error");
    // print!("{}","enter an additive key:");
    // io::stdout().flush();
    // io::stdin().read_line(&mut key2).expect("error");
    // print!("{}","enter a message:");
    // io::stdout().flush();
    // io::stdin().read_line(&mut str).expect("error");
    // let key1:i32=key1.trim().parse().unwrap();
    // let key2:i32=key2.trim().parse().unwrap();
    //
    // let cipher=affine::encrypt(&str,key1,key2);
    // println!("密文：{}",&cipher);
    // let plaintext=affine::decrypt(&cipher,key1,key2);
    // println!("原文：{}",&plaintext);

    //希尔密码
    let mut str: String = String::new();
    print!("{}", "enter a message:");
    io::stdout().flush();
    io::stdin().read_line(&mut str).expect("error");
    let mut matrix_str: String = String::new();
    print!("{}", "输入矩阵，逗号间隔开:");
    io::stdout().flush();
    io::stdin().read_line(&mut matrix_str).expect("error");
    let split: Vec<&str> = matrix_str.trim().split(',').collect();
    let mut matrix: [[i32; 2]; 2] = [[0; 2]; 2];
    matrix[0][0] = split[0].trim().parse().unwrap();
    matrix[0][1] = split[1].trim().parse().unwrap();
    matrix[1][0] = split[2].trim().parse().unwrap();
    matrix[1][1] = split[3].trim().parse().unwrap();

    let cipher = hill::encrypt(&(str.trim().to_string()), matrix);
    println!("cipher : {}", &cipher);
    let plaintext = hill::decrypt(&cipher, matrix);
    println!("plaintext : {}", &plaintext);
}
