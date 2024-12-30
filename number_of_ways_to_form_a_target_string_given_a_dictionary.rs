pub fn num_ways(words: Vec<String>, target: String) -> i32 {

    const MOD: i64 = 1_000_000_007;

    let word_length = words[0].len();
    let target = target.as_bytes();

    let mut prev = vec![0; target.len() + 1];
    *prev.last_mut().unwrap() = 1;
    let mut curr = prev.clone();

    for pos in (0..word_length).rev() {
        let mut letter_counts = vec![0; 26];
        for w in &words {
            letter_counts[(w.as_bytes()[pos] - b'a') as usize] += 1;
        }

        for idx in 0..target.len() {
            let target_match = letter_counts[(target[idx] - b'a') as usize];
            curr[idx] = (prev[idx] as i64 + target_match as i64 * prev[idx + 1] as i64).rem_euclid(MOD) as i32;
        }

        println!("curr: {curr:?}, prev: {prev:?}");
        std::mem::swap(&mut curr, &mut prev);

    }

    return prev[0];
}

#[test]
fn tests() {
    let words = vec![String::from("acca"),String::from("bbbb"),String::from("caca")];
    let target = String::from("aba");
    assert_eq!(6, num_ways(words, target));

    let words = vec![String::from("abba"),String::from("baab")];
    let target = String::from("bab");
    assert_eq!(4, num_ways(words, target));
}
