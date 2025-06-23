pub fn k_mirror(k: i32, n: i32) -> i64 {

    let mut found = 0;
    let mut ans: i64 = 0;

    let mut initial: i64 = 1;
    loop {
        for lhs in initial..(initial*10) {

            // odds lengths come first
            let mut full_value = lhs;
            let mut remaining = lhs / 10;
            while remaining > 0 {
                full_value = full_value * 10 + remaining % 10;
                remaining /= 10;
            }
            if check_other_base(full_value, k) {
                found += 1;
                ans += full_value;
            }
            
            if found == n { return ans }
        }
        for lhs in initial..(initial*10) {

            // then evens
            let mut full_value = lhs;
            let mut remaining = lhs;
            while remaining > 0 {
                full_value = full_value * 10 + remaining % 10;
                remaining /= 10;
            }
            if check_other_base(full_value, k) {
                found += 1;
                ans += full_value;
            }

            if found == n { return ans }
        }
        initial *= 10;
    }
}

fn check_other_base(mut num: i64, base: i32) -> bool {

    let mut digits = [0; 100];
    let mut length = 0;

    while num > 0 {
        digits[length as usize] = num % base as i64;
        num /= base as i64;
        length += 1;
    }
    let (mut l, mut r) = (0, length-1);
    while l < r {
        if digits[l] != digits[r] { return false; }
        l += 1;
        r -= 1;
    }
    return true;
}

#[test]
fn tests() {
    let k = 2;
    let n = 5;
    assert_eq!(25, k_mirror(k,n));

    let k = 3;
    let n = 7;
    assert_eq!(499, k_mirror(k,n));

    let k = 7;
    let n = 17;
    assert_eq!(20379000, k_mirror(k,n));
}
