pub fn range_sum(nums: Vec<i32>, n: i32, left: i32, right: i32) -> i32 {

    let n = n as usize;
    let modulus = 1_000_000_007;
    let mut ans = 0;
    
    let mut range_sums = Vec::with_capacity(n * (n+1) / 2);

    for i in 0..n {
        let mut s = 0;
        for j in i..n {
            s += nums[j];
            range_sums.push(s);
        }
    }

    range_sums.sort();

    for idx in (left as usize-1)..right as usize {
        ans = (ans % modulus) + range_sums[idx];
    }
    

    return ans;
}

#[test]
fn tests() {
    let v = vec![1,2,3,4];
    let n = 4;
    let l = 1;
    let r = 5;
    assert_eq!(13, range_sum(v,n,l,r));

    let v = vec![1,2,3,4];
    let n = 4;
    let l = 3;
    let r = 4;
    assert_eq!(6, range_sum(v,n,l,r));

    let v = vec![100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100];
    let n = 100;
    let l = 1;
    let r = 5050;
    assert_eq!(17170000, range_sum(v,n,l,r))
}
