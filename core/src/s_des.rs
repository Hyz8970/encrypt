static P10: [i32; 10] = [3, 5, 2, 7, 4, 10, 1, 9, 8, 6];
static P8: [i32; 8] = [6, 3, 7, 4, 8, 5, 10, 9];
static P4: [i32; 4] = [2, 4, 3, 1];

static IP: [i32; 8] = [2, 6, 3, 1, 4, 8, 5, 7];
static IPI: [i32; 8] = [4, 1, 3, 5, 7, 2, 8, 6];
static EP: [i32; 8] = [4, 1, 2, 3, 2, 3, 4, 1];

static S0: [[i32; 4]; 4] = [[1, 0, 3, 2], [3, 2, 1, 0], [0, 2, 1, 3], [3, 1, 3, 2]];
static S1: [[i32; 4]; 4] = [[0, 1, 2, 3], [2, 0, 1, 3], [3, 0, 1, 0], [2, 1, 0, 3]];

pub fn encrypt(text: u8, key: i16) -> u8 {
    let fk1 = fk(key);
    let i = replacement(text as i16, 8, &IP);
    let l0 = i >> 4 & 0xf;
    let r0 = i & 0xf;
    let mut r0e = replacement(r0, 4, &EP);
    r0e ^= fk1.0 as i16;
    let s1 = substitute(((r0e >> 4) & 0xf) as usize, S0);
    let s2 = substitute((r0e & 0xf) as usize, S1);
    let ss = ((0i32 | s1) << 2) as i32 | s2;
    let f1 = replacement(ss as i16, 4, &P4);
    let r1 = f1 ^ l0;
    //第二轮
    let mut r11 = replacement(r1, 4, &EP);
    r11 ^= fk1.1 as i16;
    let s1 = substitute(((r11 >> 4) & 0xf) as usize, S0);
    let s2 = substitute((r11 & 0xf) as usize, S1);
    let ss = ((0i32 | s1) << 2) as i32 | s2;
    let f2 = replacement(ss as i16, 4, &P4);
    let r2 = f2 ^ r0;
    replacement((((0i16 | r2) << 4) | r1) as i16, 8, &IPI) as u8
}

pub fn decrypt(cipher: u8, key: i16) -> u8 {
    let fk1 = fk(key);
    let i = replacement(cipher as i16, 8, &IP);
    let l0 = i >> 4 & 0xf;
    let r0 = i & 0xf;
    let mut r0e = replacement(r0, 4, &EP);
    r0e ^= fk1.1 as i16;//用第二个密钥
    let s1 = substitute(((r0e >> 4) & 0xf) as usize, S0);
    let s2 = substitute((r0e & 0xf) as usize, S1);
    let ss = ((0i32 | s1) << 2) as i32 | s2;
    let f1 = replacement(ss as i16, 4, &P4);
    let r1 = f1 ^ l0;
    //第二轮
    let mut r11 = replacement(r1, 4, &EP);
    r11 ^= fk1.0 as i16;
    let s1 = substitute(((r11 >> 4) & 0xf) as usize, S0);
    let s2 = substitute((r11 & 0xf) as usize, S1);
    let ss = ((0i32 | s1) << 2) as i32 | s2;
    let f2 = replacement(ss as i16, 4, &P4);
    let r2 = f2 ^ r0;
    replacement((((0i16 | r2) << 4) | r1) as i16, 8, &IPI) as u8
}

//生成两个密钥
fn fk(key: i16) -> (i8, i8) {
    let i = replacement(key, 10, &P10);
    let mut lk = (i >> 5) & 0b11111;
    let mut rk = i & 0b11111;
    lk = cyclic_shift_left(lk as u8, 1) as i16;
    rk = cyclic_shift_left(rk as u8, 1) as i16;
    let key1 = replacement((((0i16 | lk) << 5) | rk) & 0x3ff, 10, &P8);
    println!("key1:{:?}", format!("{:#010b}", &key1));
    lk = cyclic_shift_left(lk as u8, 2) as i16;
    rk = cyclic_shift_left(rk as u8, 2) as i16;
    let key2 = replacement((((0i16 | lk) << 5) | rk) & 0x3ff, 10, &P8);
    println!("key2:{:?}", format!("{:#010b}", &key2));
    (key1 as i8, key2 as i8)
}

//置换
fn replacement(num: i16, length: i32, p: &[i32]) -> i16 {
    let len = p.len();
    let mut result = 0i16;
    for l in 0..len {
        result <<= 1;
        result |= (num >> (length - p[l])) & 1;
    }
    result
}

//盒替代
fn substitute(num: usize, p: [[i32; 4]; 4]) -> i32 {
    p[((num >> 2) & 2) | (num & 1)][(num >> 1) & 3].clone()
}

//循环位移
fn cyclic_shift_left(num: u8, n: i32) -> u8 {
    ((num << n) | (num >> (5 - n))) & 0b00011111
}

fn cyclic_shift_right(num: u8, n: i32) -> u8 {
    ((num >> n) | (num << (5 - n))) & 0b00011111
}