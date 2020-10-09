pub mod hill;
pub mod affine;
pub mod virginia;

#[cfg(test)]
mod tests {
    use crate::affine;
    use crate::hill;
    use crate::virginia;

    //仿射密码
    #[test]
    fn affine() {
        let str: String = "aliceb".to_string();
        let cipher = affine::encrypt(&str, 7, 6);
        println!("{}", &cipher);
        let plaintext = affine::decrypt(&cipher, 7, 6);
        println!("{}", &plaintext);
        assert_eq!(str, plaintext)
    }

    //希尔密码
    #[test]
    fn hill() {
        let str: String = "sundays".to_string();//[[11, 8], [3, 7]]
        let cipher = hill::encrypt(&str, [[9, 6], [7, 11]]);
        println!("cipher : {}", &cipher);
        let plaintext = hill::decrypt(&cipher, [[9, 6], [7, 11]]);
        println!("plaintext : {}", &plaintext);
        assert_eq!(str, plaintext);
    }

    //维吉利亚密码
    #[test]
    fn virginia() {
        let str: String = "cloudsecurity".to_string();
        let cipher = virginia::encrypt(&str, "alice".to_string());
        println!("{}",&cipher);
        let plaintext = virginia::decrypt(&cipher, "alice".to_string());
        println!("{}",&plaintext);
        assert_eq!(str,plaintext);
    }

    //S-DES密码
    #[test]
    fn s_des(){

    }
}
