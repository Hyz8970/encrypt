pub fn encrypt(text: &String, matrix: [[i32; 2]; 2]) -> String {
    let mut flag = 0;
    let mut temp = text.to_lowercase();
    let mut len = text.len();
    let mut str: Vec<i32> = vec![];
    if len % 2 == 1 {
        temp.push('a');
        len = temp.len();
        flag = 1;
    }
    for t in temp.chars() {
        let tt = t as i32;
        str.push(tt - 97);
    }
    let mut arr: Vec<i32> = vec![];
    for j in (0..len).step_by(2) {
        let t1 = match str.get(j) {
            None => 0,
            Some(l) => *l,
        };
        let t2 = match str.get(j + 1) {
            None => 0,
            Some(l) => *l,
        };
        arr.push((t1 * matrix[0][0] + t2 * matrix[1][0]) % 26);
        arr.push((t1 * matrix[0][1] + t2 * matrix[1][1]) % 26);
    }
    if 1 == flag {
        arr.push(-36);//奇数加等号标识
    }
    let mut result = String::new();
    for a in arr {
        result.push(char::from((a + 97) as u8));
    }
    result
}

pub fn decrypt(cipher: &String, matrix: [[i32; 2]; 2]) -> String {
    let mut flag = 0;
    let mut temp = cipher.to_lowercase();
    let mut len = cipher.len();
    let mut str: Vec<i32> = vec![];
    if len % 2 == 1 {
        temp.pop();//去掉等号
        len = temp.len();
        flag = 1;
    }
    for t in temp.chars() {
        let tt = t as i32;
        str.push(tt - 97);
    }
    //求逆
    let mut te = -1;
    for i in 1.. {
        te = (matrix[0][0] * matrix[1][1] - matrix[0][1] * matrix[1][0]) + 26 * i;
        if 0 <= te { break; }
    }
    for j in 1.. {
        if 1 == (te * j) % 26 {
            te = j;
            break;
        }
    }
    let mut inv_matrix: [[i32; 2]; 2] = [[0; 2]; 2];
    inv_matrix[0][0] = matrix[1][1] * te;
    inv_matrix[0][1] = ((-1 * matrix[0][1] + 26) * te) % 26;
    inv_matrix[1][0] = ((-1 * matrix[1][0] + 26) * te) % 26;
    inv_matrix[1][1] = matrix[0][0] * te;

    let mut arr: Vec<i32> = vec![];
    for j in (0..len).step_by(2) {
        let t1 = match str.get(j) {
            None => 0,
            Some(l) => *l,
        };
        let t2 = match str.get(j + 1) {
            None => 0,
            Some(l) => *l,
        };
        arr.push((t1 * inv_matrix[0][0] + t2 * inv_matrix[1][0]) % 26);
        arr.push((t1 * inv_matrix[0][1] + t2 * inv_matrix[1][1]) % 26);
    }
    if 1 == flag {
        arr.pop();
    }
    let mut result = String::new();
    for a in arr {
        result.push(char::from((a + 97) as u8));
    }
    result
}