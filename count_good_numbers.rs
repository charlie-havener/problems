pub fn count_good_numbers(n: i64) -> i32 {
    
    const MOD: i64 = 1_000_000_007;
    
    let num_pairs = n/2;
    println!("num_pairs: {num_pairs}");
    let mut ans = log_exp(20, num_pairs, MOD);

    if n%2 == 1 {
        ans = (ans * 5).rem_euclid(MOD)
    }

    return ans as i32;
}



fn log_exp(base: i32, mut exponent: i64, m: i64) -> i64 {
    
    let mut base = base as i64;
    let mut ret = 1;
    while exponent > 0 {
        if exponent & 1 == 1 {
            ret = (ret * base).rem_euclid(m);
        }
        base = (base * base).rem_euclid(m);
        exponent >>= 1;
    }

    return ret;
}

#[test]
fn tests() {
    let n = 1;
    assert_eq!(5, count_good_numbers(n));

    let n = 4;
    assert_eq!(400, count_good_numbers(n));

    let n = 50;
    assert_eq!(564908303, count_good_numbers(n));
}
