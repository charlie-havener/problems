pub fn max_count(mut banned: Vec<i32>, n: i32, max_sum: i32) -> i32 {
    banned.sort();
    let mut sum = 0;
    let mut count = 0;

    let mut bi = 0;
    for i in 1..=n {
        if bi >= banned.len() || i < banned[bi] {
            if sum + i > max_sum {
                break;
            }
            count += 1;
            sum += i;
        }
        else {
            while bi < banned.len() && i == banned[bi] {
                bi += 1;
            }
        }
    }
    return count;
}

#[test]
fn tests() {
    let banned = vec![1,6,5];
    let n = 5;
    let max_sum = 6;
    assert_eq!(2, max_count(banned, n, max_sum));

    let banned = vec![1,2,3,4,5,6,7];
    let n = 8;
    let max_sum = 1;
    assert_eq!(0, max_count(banned, n, max_sum));

    let banned = vec![11];
    let n = 7;
    let max_sum = 50;
    assert_eq!(7, max_count(banned, n, max_sum));

    let banned = vec![1,2,3,4,5,5,6,7];
    let n = 8;
    let max_sum = 1;
    assert_eq!(0, max_count(banned, n, max_sum));
}
