pub fn number_of_patterns(m: i32, n: i32) -> i32 {

    fn recurse(curr: usize, mask: i32, low: i32, high: i32, count: i32, ans: &mut i32, jumps: &Vec<Vec<i32>>) {
        if count > high { return }
        if count >= low { *ans += 1 }

        for i in 1..=9 {
            let j = jumps[curr][i];
            let visited = (mask & 1<<i) == 1<<i;
            if !visited && (j == 0 || (mask & 1<<j) == 1<<j) {
                recurse(i, mask^(1<<i), low, high, count+1, ans, jumps);
            }
        }
    }


    let mut jumps = vec![vec![0; 10];10];
    // vertical jumps
    jumps[1][3] = 2;
    jumps[3][1] = 2;
    jumps[4][6] = 5;
    jumps[6][4] = 5;
    jumps[7][9] = 8;
    jumps[9][7] = 8;

    // horizontal jumps
    jumps[1][7] = 4;
    jumps[7][1] = 4;
    jumps[2][8] = 5;
    jumps[8][2] = 5;
    jumps[3][9] = 6;
    jumps[9][3] = 6;

    // diagonal jumps
    jumps[1][9] = 5;
    jumps[9][1] = 5;
    jumps[3][7] = 5;
    jumps[7][3] = 5;
    
    let mut center = 0;
    let mask = 1 << 5;
    recurse(5, mask, m, n, 1, &mut center, &jumps);

    let mut corner = 0;
    let mask = 1 << 1;
    recurse(1, mask, m, n, 1, &mut corner, &jumps);

    let mut edge = 0;
    let mask = 1 << 2;
    recurse(2, mask, m, n, 1, &mut edge, &jumps);
    
    return center + corner*4 + edge*4;
}

#[test]
fn tests() {
    assert_eq!(9, number_of_patterns(1, 1));
    assert_eq!(65, number_of_patterns(1, 2));
    assert_eq!(389497, number_of_patterns(1,9));
    assert_eq!(72912, number_of_patterns(7,7));
}
