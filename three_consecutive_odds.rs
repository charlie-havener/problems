pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {

    let mut count = 0;
    for v in arr {
        match v%2 {
            1 => count += 1,
            0 => count = 0,
            _ => unreachable!(),
        }
        if count == 3 {
            return true;
        }
    }
    return false;
}

#[test]
fn tests() {
    let arr = vec![2,6,4,1];
    assert_eq!(false, three_consecutive_odds(arr));

    let arr = vec![1,2,34,3,4,5,7,23,12];
    assert_eq!(true, three_consecutive_odds(arr));
}
