pub fn num_teams(rating: Vec<i32>) -> i32 {
    
    let mut ans = 0;
    let l = rating.len();

    for m in 0..l {
        let mid = rating[m];
        let mut left = 0;
        let mut right = 0;
        for idx in 0..m {
            if rating[idx] < mid {
                left += 1;
            }
        }
        for idx in (m+1)..l {
            if rating[idx] > mid {
                right += 1;
            }
        }
        ans += left * right;

        // number greater than mid on the lift is the difference between that
        // sides length and the number small. same thing for the right
        ans += (m - left) * (l - m - 1 - right);
    }

    return ans as i32;
}

#[test]
fn tests() {
    let r = vec![2,5,3,4,1];
    assert_eq!(3, num_teams(r));

    let r = vec![2,1,3];
    assert_eq!(0, num_teams(r));

    let r = vec![1,2,3,4];
    assert_eq!(4, num_teams(r));
}
