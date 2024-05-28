pub fn equal_substring(s: String, t: String, max_cost: i32) -> i32 {
    let mut diffs = vec![0; s.len()];
    let mut s_b = s.chars();
    let mut t_b = t.chars();
    for i in 0..diffs.len() {
        diffs[i] = (s_b.next().unwrap() as i32 - t_b.next().unwrap() as i32).abs();
    }

    let mut left = 0;
    let mut right = 0;
    let mut total = 0;
    let mut ans = 0;

    while right < diffs.len() {
        total += diffs[right];
        right += 1;

        if total <= max_cost {
            ans = ans.max(right - left);
        }
        else {
            while total > max_cost {
                if left == right { break }
                total -= diffs[left];
                left += 1;
            }
        }
    }

    return ans as i32;
}

#[test]
fn tests() {
    let s = String::from("abcd");
    let t = String::from("bcdf");
    let m = 3;
    assert_eq!(3, equal_substring(s,t,m));
    
    let s = String::from("abcd");
    let t = String::from("cdef");
    let m = 3;
    assert_eq!(1, equal_substring(s,t,m));

    let s = String::from("abcd");
    let t = String::from(String::from("acde"));
    let m = 0;
    assert_eq!(1, equal_substring(s,t,m));

    let s = String::from("krpgjbjjznpzdfy");
    let t = String::from(String::from("nxargkbydxmsgby"));
    let m = 14;
    assert_eq!(4, equal_substring(s,t,m));
}
