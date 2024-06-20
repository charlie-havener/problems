pub fn min_days(bloom_day: Vec<i32>, m: i32, k: i32) -> i32 {
    let mut high = i32::MIN;
    let mut low = i32::MAX;
    for b in &bloom_day {
        if *b > high {
            high = *b;
        }
        if *b <low {
            low = *b;
        }
    }

    let search = | mid: i32 | -> bool {
        let mut bouquets = 0;
        let mut consecutive = 0;
        for b in &bloom_day {
            if *b <= mid {
                consecutive += 1;
            } else {
                consecutive = 0;
            }
            if consecutive == k {
                bouquets += 1;
                consecutive = 0;
            }
            if bouquets == m {
                return true;
            }
        }
        return false;
        
    };

    let mut ans = None;
    while high >= low {
        let mid = (high - low)/2 + low;
        match search(mid) {
            true => {
                ans = Some(mid);
                high = mid.wrapping_sub(1);
            },
            false => low = mid + 1,
        }
    }
    return ans.unwrap_or(-1);
}

#[test]
fn tests() {
    let b = vec![1,10,3,10,2];
    let m = 3;
    let k = 1;
    assert_eq!(3, min_days(b,m,k));

    let b = vec![1,10,3,10,2];
    let m = 3;
    let k = 2;
    assert_eq!(-1, min_days(b,m,k));

    let b = vec![7,7,7,7,12,7,7];
    let m = 2;
    let k = 3;
    assert_eq!(12, min_days(b,m,k));
}
