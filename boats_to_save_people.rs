pub fn num_rescue_boats(mut people: Vec<i32>, limit: i32) -> i32 {
    people.sort_unstable();
    let mut ans = 0;
    let (mut l, mut r) = (1, people.len());
    while l <= r {
        ans += 1;
        if people[l-1] + people[r-1] <= limit {
            l += 1;
        }
        r -= 1;

    }
    return ans;
}

#[test]
fn tests() {
    let p = vec![1,2];
    let w = 3;
    assert_eq!(1, num_rescue_boats(p, w));

    let p = vec![3,2,2,1];
    let w = 3;
    assert_eq!(3, num_rescue_boats(p, w));

    let p = vec![3,5,3,4];
    let w = 5;
    assert_eq!(4, num_rescue_boats(p, w));
}
