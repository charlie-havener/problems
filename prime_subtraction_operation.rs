pub fn prime_sub_operation(mut nums: Vec<i32>) -> bool {

    // get all the primes up to 1_000
    let mut sieve: Vec<bool> = vec![true; 500];
    for x in (3..32).step_by(2) {
        let idx = (x - 1) / 2;
        if sieve[idx] {
            let mut sift = x * x;
            while sift < 1_000 {
                sieve[(sift - 1) / 2] = false;
                sift += 2 * x;
            }
        }
    }

    let mut primes = vec![0,2];
    for idx in 1..sieve.len() {
        if sieve[idx] {
            let v = idx * 2 + 1;
            primes.push(v as i32);
        }
    }
    
    // apply max decrement to each element in nums, return false on fail
    let mut prev = 0;
    for n in nums {

        // binary search for the largest prime that can be subtracted without going under prev
        // left will be the last element <= search_value
        // right will be the first elemnt > search_value
        // search value has the -1 to make sure that subtracting left is allowed (even when = )
        let search_value = (n - prev) - 1;
        let (mut left, mut right): (i32, i32) = (-1, primes.len() as i32);
        while right - left > 1 {
            let mid = (right + left) / 2;
            if search_value < primes[mid as usize] { right = mid }
            else { left = mid }
        }

        if left == -1 { return false }
        prev = n - primes[left as usize];

    }

    return true;
}

#[test]
fn tests() {
    let nums = vec![4,9,6,10];
    assert_eq!(true, prime_sub_operation(nums));

    let nums = vec![6,8,11,12];
    assert_eq!(true, prime_sub_operation(nums));

    let nums = vec![5,8,3];
    assert_eq!(false, prime_sub_operation(nums));
}
