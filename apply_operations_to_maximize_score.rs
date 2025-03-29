use std::collections::HashSet;

pub fn maximum_score(nums: Vec<i32>, k: i32) -> i32 {

    let max_element = *nums.iter().max().unwrap();
    let lowest_prime_factors = lowest_prime_factors(max_element);

    // can get prime factorization of n in O(number of factors) time
    // store factors in hashset to get distinct count.
    // since n <= 100_000, there are at most 6 distinct prime factors
    let mut factors = Vec::with_capacity(nums.len());
    let mut factor_set = HashSet::with_capacity(6);
    for n in &nums {
        let mut curr = *n as usize;
        while curr > 1 {
            let f = lowest_prime_factors[curr];
            factor_set.insert(f);
            curr /= f;
        }
        factors.push(factor_set.len());
        factor_set.clear();
    }

    // monotonic stacks for determining number of subarrays
    // where each number would be chosen
    let mut nxt = vec![nums.len() as i32; nums.len()];
    let mut prv = vec![-1; nums.len()];
    let mut stack = Vec::new();
    for i in 0..nums.len() {
        while !stack.is_empty() {
            if factors[*stack.last().unwrap()] < factors[i] {
                let t = stack.pop().unwrap();
                nxt[t] = i as i32;
            } else {
                break;
            }
        }
        if !stack.is_empty() {
            prv[i] = *stack.last().unwrap() as i32;
        }

        stack.push(i);
    }

    // use the nxt and prv to determine how many subarrays
    // whould have each n seleceted
    // store n and number of subarrays in an array
    // sort that array base on n and select top k times
    let mut store: Vec<(i32, i64)> = Vec::with_capacity(nums.len());
    for i in 0..nums.len() {
        store.push((nums[i], (nxt[i] - i as i32) as i64 * (i as i32 - prv[i]) as i64));
    }
    store.sort_unstable_by_key(|(a, _)| std::cmp::Reverse(*a));

    let mut ans: i64 = 1;
    let mut k = k as i64;
    const MOD: i64 = 1_000_000_007;
    for s in store {
        ans = (ans * power(s.0 as i64, k.min(s.1) as i64, MOD)).rem_euclid(MOD);
        k -= s.1;
        if k <= 0 { return ans as i32 }
    }

    unreachable!();
}

fn power(mut base: i64, mut power: i64, m: i64) -> i64 {
    let mut ans = 1;
    while power > 0 {
        if power % 2 == 1 {
            ans = (ans * base).rem_euclid(m);
        }
        base = (base * base).rem_euclid(m);
        power /= 2;
    }
    return ans;
}

fn lowest_prime_factors(n: i32) -> Vec<usize> {
    let n = n as usize;
    let mut lowest_prime_factor = vec![0; n+1];
    let mut primes = Vec::with_capacity(1000);

    for i in 2..=n {
        if lowest_prime_factor[i] == 0 {
            lowest_prime_factor[i] = i;
            primes.push(i);
        }
        let mut j = 0;
        while i * primes[j] <= n {
            lowest_prime_factor[i * primes[j]] = primes[j];
            if primes[j] == lowest_prime_factor[i] {
                break;
            }
            j += 1;
        }
    }

    return lowest_prime_factor;
}

#[test]
fn tests() {

    

    let nums = vec![8,3,9,3,8];
    let k = 2;
    assert_eq!(81, maximum_score(nums, k));

    let nums = vec![19,12,14,6,10,18];
    let k = 3;
    assert_eq!(4788, maximum_score(nums, k));

    let nums = vec![31,210,21,60,6,5,7,14];
    let k = 28;
    assert_eq!(278107243, maximum_score(nums, k));
}
