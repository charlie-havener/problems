pub fn find_kth_number(n: i32, k: i32) -> i32 {
    
    let step_count = |mut p1: i32, mut p2: i32| -> i32 {
        let mut count = 0;
        while p1 <= n {
            count += (n+1).min(p2) - p1;
            p1 *= 10;
            p2 *= 10;
        }
        return count;
    };

    let mut curr = 1;
    let mut k = k-1;

    while k > 0 {
        let s = step_count(curr, curr+1);
        if k >= s {
            k -= s;
            curr += 1;
        }
        else {
            curr *= 10;
            k -= 1;
        }
    }

    return curr;
}

#[test]
fn tests() {

    let order = vec![1,10,11,12,13,14,15,16,17,18,19,2,20,21,22,23,24,25,26,27,28,29,3,30,4,5,6,7,8,9];

    let n = 30;
    for k in 1..=n {
        assert_eq!(order[k as usize - 1], find_kth_number(n, k));
    }
}
