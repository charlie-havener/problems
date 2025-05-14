const MOD: i64 = 1_000_000_007;

pub fn length_after_transformations(s: String, t: i32, nums: Vec<i32>) -> i32 {

    let mut trans = [[0; 26]; 26];
    for i in 0..nums.len() {
        for mut j in (i+1)..=(i+nums[i] as usize) {
            if j >= 26 { j -= 26 }
            trans[j][i] = 1;
        }
    }
    println!("{trans:?}");


    let mut counts = [0; 26];
    for c in s.as_bytes() {
        counts[(c - b'a') as usize] += 1;
    }


    // binary exponentiation of trans
    let mut mat = [[0; 26]; 26];
    for i in 0..26 {
        mat[i][i] = 1;
    }
    let mut curr = trans.clone();
    let mut exp = t;
    while exp > 0 {
        if exp & 1 == 1 {
            mat = matrix_multiply(&mat, &curr);
        }
        curr = matrix_multiply(&curr, &curr);
        exp >>= 1;
    }

    let mut ans = 0_i64;
    for i in 0..26 {
        for j in 0..26 {
            ans = (ans + mat[i][j] * counts[j]) % MOD;
        }
        println!("{:?}, {:?}", (b'a' + i as u8) as char, ans);
    }

    return ans as i32;
}


fn matrix_multiply(a: &[[i64; 26]; 26], b: &[[i64; 26]; 26]) -> [[i64; 26]; 26] {
    let mut r = [[0_i64;26]; 26];
    for i in 0..26 {
        for j in 0..26 {
            for k in 0..26 {
                r[i][j] = (r[i][j] + a[i][k] * b[k][j]) % MOD;
            }
        }
    }
    return r;
}


#[test]
fn tests() {
    let s = String::from("abcyy");
    let t = 2;
    let nums = vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2];
    println!("{:?}", length_after_transformations(s, t, nums));

    let s = String::from("abcyy");
    let t = 2225;
    let nums = vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2];
    println!("{:?}", length_after_transformations(s, t, nums));
}
