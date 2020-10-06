pub mod hill;
pub mod affine;
pub mod virginia;
#[cfg(test)]
mod tests {
    use crate::affine;

    //仿射密码
    #[test]
    fn affine() {
        let str:String="alice".to_string();
        let cipher=affine::encrypt(&str,7,6);
        println!("{}",&cipher);
        let plaintext=affine::decrypt(&cipher,7,6);
        println!("{}",&plaintext);
        assert_eq!(str,plaintext)
    }

    //希尔密码
    #[test]
    fn hill(){
        let str :String="se".to_string();
    }

    //维吉利亚密码
    #[test]
    fn virginia(){
        let str:String="cloudsecurity".to_string();

    }
}
