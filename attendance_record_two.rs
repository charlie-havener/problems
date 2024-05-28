pub fn check_record(mut n: i32) -> i32 {

    let modulus: i32 = 1_000_000_007;
    let mut curr = [[1,1],[1,0],[0,0]];
    while n > 1 {

        let mut next = [[0,0],[0,0],[0,0]];

        //0A, 0L - curr[0][0]
        next[0][0] = (next[0][0] + curr[0][0]) % modulus;
        next[0][1] = (next[0][1] + curr[0][0]) % modulus;
        next[1][0] = (next[1][0] + curr[0][0]) % modulus;
        
        //0A, 1L - curr[1][0]
        next[0][0] = (next[0][0] + curr[1][0]) % modulus;
        next[0][1] = (next[0][1] + curr[1][0]) % modulus;
        next[2][0] = (next[2][0] + curr[1][0]) % modulus;

        //0A, 2L - curr[2][0]
        next[0][0] = (next[0][0] + curr[2][0]) % modulus;
        next[0][1] = (next[0][1] + curr[2][0]) % modulus;

        //1A, 0L - curr[0][1]
        next[0][1] = (next[0][1] + curr[0][1]) % modulus; 
        next[1][1] = (next[1][1] + curr[0][1]) % modulus;

        //1A, 1L - curr[1][1]
        next[0][1] = (next[0][1] + curr[1][1]) % modulus;
        next[2][1] = (next[2][1] + curr[1][1]) % modulus;

        //1A, 2L - curr[2][1]
        next[0][1] = (next[0][1] + curr[2][1]) % modulus;

        curr = next;

        n -= 1;
    }

    let mut ans = 0;
    for row in curr {
        for v in row {
            ans = (ans + v) % modulus;
        }
    }

    return ans;
}

#[test]
fn tests() {
    let n = 1;
    assert_eq!(3, check_record(n));

    let n = 2;
    assert_eq!(8, check_record(n));

    let n = 10101;
    assert_eq!(183236316, check_record(n));
}
