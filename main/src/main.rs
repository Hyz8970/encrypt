use core::affine;
fn main() {
    //仿射密码
    let str:String="alice".to_string();
    let cipher=affine::encrypt(&str,7,6);
    println!("密文：{}",&cipher);
    let plaintext=affine::decrypt(&cipher,7,6);
    println!("原文：{}",&plaintext);
}
