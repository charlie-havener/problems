pub fn sort_jumbled(mapping: Vec<i32>, mut nums: Vec<i32>) -> Vec<i32> {
    let conv = |mut n| -> i32 {
        print!("{n}");
        let mut count = 0;
        let mut ans = 0;
        loop {
            let d = mapping[n as usize % 10];
            ans += d * i32::pow(10, count);
            count += 1;
            n /= 10;
            if n == 0 { break }
        } 
        println!(" -> {n}");
        return ans;
    };
    nums.sort_by_cached_key(|n| conv(*n));
    return nums;
}

#[test]
fn tests() {
    let m = vec![8,9,4,0,2,1,3,5,7,6];
    let n = vec![991,338,38];
    assert_eq!(vec![338,38,991], sort_jumbled(m,n));

    let m = vec![0,1,2,3,4,5,6,7,8,9];
    let n = vec![789,456,123];
    assert_eq!(vec![123,456,789], sort_jumbled(m,n));

    let m = vec![9,8,7,6,5,4,3,2,1,0];
    let n = vec![0,1,2,3,4,5,6,7,8,9];
    assert_eq!(vec![9,8,7,6,5,4,3,2,1,0], sort_jumbled(m,n));
}
