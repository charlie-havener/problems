pub fn kth_lucky_number(k: i32) -> String {
    let k = k as i64;
    let mut low = 0;
    let mut high = 2;
    let mut digits = 1;
    while high < k {
        std::mem::swap(&mut low, &mut high);
        high = low + 2*(low - high);
        digits += 1;
    }

    let mut ans = String::with_capacity(digits);
    for idx in 0..digits {
        let mid = (high - low) / 2 + low;
        if mid >= k {
            high = mid;
            ans.push('4');
        } else {
            low = mid;
            ans.push('7');
        }
    }

    return ans;
}

#[test]
fn tests() {
    let k = 4;
    assert_eq!(String::from("47"), kth_lucky_number(k));

    let k = 10;
    assert_eq!(String::from("477"), kth_lucky_number(k));

    let k = 100;
    assert_eq!(String::from("744747"), kth_lucky_number(k));

    let k = 1000;
    assert_eq!(String::from("777747447"), kth_lucky_number(k));

    let k = 125643;
    assert_eq!(String::from("7774747477447744"), kth_lucky_number(k));

    let k = 7;
    assert_eq!(String::from("444"), kth_lucky_number(k));
    let k = 8;
    assert_eq!(String::from("447"), kth_lucky_number(k));
    let k = 9;
    assert_eq!(String::from("474"), kth_lucky_number(k));
    let k = 10;
    assert_eq!(String::from("477"), kth_lucky_number(k));
    let k = 11;
    assert_eq!(String::from("744"), kth_lucky_number(k));
    let k = 12;
    assert_eq!(String::from("747"), kth_lucky_number(k));
    let k = 13;
    assert_eq!(String::from("774"), kth_lucky_number(k));
    let k = 14;
    assert_eq!(String::from("777"), kth_lucky_number(k));
}
