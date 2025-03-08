pub fn closest_primes(left: i32, right: i32) -> Vec<i32> {

    if left <= 2 && right >= 3 { return vec![2,3]; }

    let mut ans = vec![0, i32::MAX];
    let mut primes: Vec<bool> = vec![true; (right as usize + 1) / 2];

    let mut curr = vec![-1,-1];
    let mut n = 3;

    let check = |n: i32, curr: &mut Vec<i32>, ans: &mut Vec<i32>| {
        if n >= left {
            curr[0] = curr[1];
            curr[1] = n;
            if curr[0] != -1 && ans[1] - ans[0] > curr[1] - curr[0] {
                ans[0] = curr[0];
                ans[1] = curr[1];
            }
        }
    };

    while n * n <= right {
        if ans[1] - ans[0] == 2 { return ans; }
        if primes[(n/2) as usize] {
            check(n, &mut curr, &mut ans);
        }

        let mut c = n + 2*n;
        while c <= right {
            primes[(c/2) as usize] = false;
            c += 2*n;
        }
        
        n += 2;
    }

    //let p = primes.iter().enumerate().filter(|v| *v.1 == true).map(|(idx, _)| idx * 2 + 1).collect::<Vec<usize>>();
    //println!("{p:?}");
    //println!("{:?}", primes);

    while n <= right {
        if primes[(n/2) as usize] {
            check(n, &mut curr, &mut ans);
        }
        n += 2;
    }


    if ans[0] == i32::MAX || ans[1] == i32::MAX {
        return vec![-1,-1];
    }
    return ans;
}

#[test]
fn tests() {
    println!("{:?}", closest_primes(10,19));
    println!("{:?}", closest_primes(4,6));
    println!("{:?}", closest_primes(3,5));
    println!("{:?}", closest_primes(1,8));
    println!("{:?}", closest_primes(26,41));
    println!("{:?}", closest_primes(415380,416522));
}
