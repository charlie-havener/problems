pub fn maximum_candies(candies: Vec<i32>, k: i64) -> i32 {

    let mut ans = 0;
    let large = *candies.iter().max().unwrap();
    let (mut left, mut right) = (1, large);

    let is_possible = |target: i32| -> bool {
        let mut piles: i64 = 0;
        for &c in &candies {
            piles += (c/target) as i64;
        }
        if piles >= k {
            return true;
        }
        return false;
    };

    while left <= right {
        
        let mid = (left + right) / 2;
        if is_possible(mid) {
            ans = ans.max(mid);
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    return ans;
}
