pub fn longest_palindrome(words: Vec<String>) -> i32 {

    let mut ans = 0;
    let mut counts = [[0; 26]; 26];
    for w in words.iter().map(|w| w.as_bytes()) {
        let (a,b) = ((w[0] - b'a') as usize, (w[1] - b'a') as usize);
        counts[a][b] += 1;
    }

    // diagonals are already palindromes (aa, bb, ..., zz)
    // so if one of them has an odd count, then it could go in the middle
    // but that can only happen once, so just keep a flag if that happens
    // and add 2 to the answer if the flag is set.
    let mut flag = false;
    for d in 0..=25 {
        if counts[d][d] & 1 == 1 {
            flag = true;
        }
        ans += (counts[d][d] >> 1) * 4;
    }
    if flag { ans += 2 }

    // the matching pair to [r][c] is [c][r]. Add min of those two values * 4 to ans
    for row in 0..=24 {
        for col in (row+1)..=25 {
            ans += counts[row][col].min(counts[col][row]) * 4;
        }
    }

    return ans;
}
