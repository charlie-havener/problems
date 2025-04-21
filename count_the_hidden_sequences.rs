pub fn number_of_arrays(differences: Vec<i32>, lower: i32, upper: i32) -> i32 {
    let mut t: i64 = 0;
    let mut small: i64 = 0;
    let mut large: i64 = 0;
    for d in differences {
        t += d as i64;
        small = t.min(small);
        large = t.max(large);
    }
    return (small - lower as i64 + upper as i64 - large + 1).max(0) as i32;
}

#[test]
fn tests() {

    let differences = vec![1,-3,4];
    let lower = 1;
    let upper = 6;
    assert_eq!(2, number_of_arrays(differences, lower, upper));

    let differences = vec![3,-4,5,1,-2];
    let lower = -4;
    let upper = 5;
    assert_eq!(4, number_of_arrays(differences, lower, upper));

    let differences = vec![4,-7,2];
    let lower = 3;
    let upper = 6;
    assert_eq!(4, number_of_arrays(differences, lower, upper));

}
