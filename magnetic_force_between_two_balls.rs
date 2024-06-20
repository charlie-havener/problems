pub fn max_distance(mut position: Vec<i32>, m: i32) -> i32 {
    
    position.sort_unstable();

    let mut left = 1;
    let mut right = *position.iter().max().unwrap() / (m-1);

    let check = | mid: i32 | -> bool {
        let mut balls = m - 1;

        let mut last = position[0];
        for p in &position[1..] {
    
            if p - last >= mid {
                last = *p;
                balls -= 1;
            }

            if balls == 0 {
                return true;
            }
        }
        return false;
    };


    let mut ans = 1;
    while left <= right {

        let mid = (right - left)/2 + left;
        match check(mid) {
            true => {
                ans = ans.max(mid);
                left = mid + 1;
            },
            false => right = mid.wrapping_sub(1),
        }
    }

    return ans;
}

#[test]
fn tests() {
    let p = vec![1,2,3,4,7];
    let m = 3;
    assert_eq!(3, max_distance(p,m));

    let p = vec![5,4,3,2,1,1000];
    let m = 2;
    assert_eq!(999, max_distance(p,m));
}
