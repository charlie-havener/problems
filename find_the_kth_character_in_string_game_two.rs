pub fn kth_character(k: i64, operations: Vec<i32>) -> char {
    
    let mut increments = 0;
    let mut k = k-1;
    let mut pos = 0;

    while k > 0 {
        if k & 1 == 1 {
            increments += operations[pos];
        }
        pos += 1;
        k >>= 1;
    }

    return (b'a' + (increments % 26) as u8) as char;

}

#[test]
fn tests() {
    let k = 5;
    let operations = vec![0,0,0];
    assert_eq!('a', kth_character(k, operations));

    let k = 10;
    let operations = vec![0,1,0,1];
    assert_eq!('b', kth_character(k, operations));

    let k = 10000000678000;
    let operations = vec![0,1,0,1,1,1,1,0,0,0,1,0,0,1,1,0,1,1,1,1,0,0,1,0,1,0,1,0,0,0,0,0,1,1,1,0,1,0,0,1,1,0,1,1,0,1,0,0,0,0,1,1,1,0,1,1,1,1,0,0,1,0,1,0,1,1,1,1,1,0,0,0,1,1,0,0,1,1,0,0,1,1,1,1,1,0,0,0,0,0,1,1,1,0,0,1,1,0,0,0];
    assert_eq!('n', kth_character(k, operations));
}
