pub fn chalk_replacer(chalk: Vec<i32>, k: i32) -> i32 {

    let mut sum = 0;
    for (idx, v) in chalk.iter().enumerate() {
        sum += v;
        if sum > k {
            return idx as i32;
        }
    }

    let mut k = k%sum;
    for (idx, v) in chalk.iter().enumerate() {
        k -= v;
        if k < 0 {
            return idx as i32;
        }
    }

    unreachable!();
}

#[test]
fn tests() {
    let c = vec![5,1,5];
    let k = 22;
    assert_eq!(0, chalk_replacer(c, k));

    let c = vec![3,4,1,2];
    let k = 25;
    assert_eq!(1, chalk_replacer(c, k));
}
