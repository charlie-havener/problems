pub fn shortest_common_supersequence(str1: String, str2: String) -> String {

    let mut dp = vec![vec![0; str2.len() + 1]; str1.len() + 1];

    let s1 = str1.as_bytes();
    let s2 = str2.as_bytes();

    for c in 1..dp[0].len() {
        dp[0][c] = c;
    }
    for r in 1..dp.len() {
        dp[r][0] = r;
    }

    for r in 1..dp.len() {
        for c in 1..dp[0].len() {
            if s1[r-1] == s2[c-1] {
                dp[r][c] = dp[r-1][c-1] + 1;
            }
            else {
                dp[r][c] = dp[r][c-1].min(dp[r-1][c]) + 1;
            }
        }
    }
    
    let (mut r, mut c) = (dp.len() - 1, dp[0].len() - 1);
    let mut ptr = dp[r][c] - 1;
    let mut ans = vec![0; dp[r][c]];
    while r > 0 && c > 0 {
        if s1[r - 1] == s2[c - 1] {
            ans[ptr] = s1[r-1];
            r -= 1;
            c -= 1;
        } else {
            if dp[r-1][c] < dp[r][c-1] {
                ans[ptr] = s1[r-1];
                r -= 1;
            }
            else {
                ans[ptr] = s2[c-1];
                c -= 1;
            }
        }
        ptr -= 1;
    }

    println!("h");
    if r == 0 {
        while c > 0 {
            ans[ptr] = s2[c-1];
            c -= 1;
            if ptr == 0 { break }
            ptr -= 1;
        }
    }

    if c == 0 {
        while r > 0 {
            ans[ptr] = s1[r-1];
            r -= 1;
            if ptr == 0 { break }
            ptr -= 1;
        }
    }


    return String::from_utf8(ans).unwrap();
}

#[test]
fn tests() {
    //let str1 = String::from("abac");
    //let str2 = String::from("cab");
    //assert_eq!("cabac", shortest_common_supersequence(str1, str2));

    let str1 = String::from("cab");
    let str2 = String::from("abac");
    assert_eq!("cabac", shortest_common_supersequence(str1, str2));
}
