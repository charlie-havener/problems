pub fn lucky_numbers(matrix: Vec<Vec<i32>>) -> Vec<i32> {

    let mut r_value = i32::MIN;
    for r in 0..matrix.len() {
        let mut rv = i32::MAX;
        for c in 0..matrix[0].len() {
            rv = rv.min(matrix[r][c]);
        }
        r_value = r_value.max(rv);
    }

    let mut c_value = i32::MAX;
    for c in 0..matrix[0].len() {
        let mut cv = i32::MIN;
        for r in 0..matrix.len() {
            cv = cv.max(matrix[r][c]);
        }
        c_value = c_value.min(cv);
    }

    if r_value == c_value { return vec![r_value] }
    return vec![];


}

#[test]
fn tests() {
    let m = vec![vec![3,7,8],vec![9,11,13],vec![15,16,17]];
    assert_eq!(vec![15], lucky_numbers(m)); 

    let m = vec![vec![1,10,4,2],vec![9,3,8,7],vec![15,16,17,12]];
    assert_eq!(vec![12], lucky_numbers(m)); 

    let m = vec![vec![7,8],vec![1,2]];
    assert_eq!(vec![7], lucky_numbers(m)); 
}
