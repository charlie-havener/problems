pub fn count_of_substrings(word: String, k: i32) -> i64 {

    let word = word.as_bytes();
    countsK(word, k + 1) - countsK(word, k)

}

fn countsK(word: &[u8], k: i32) -> i64 {

    let is_vowel = |c: u8| -> Option<usize> {
        match c {
            b'a' => Some(0),
            b'e' => Some(1),
            b'i' => Some(2),
            b'o' => Some(3),
            b'u' => Some(4),
            _ => None,
        }
    };

    let mut count: i64 = 0;
    let mut s = 0;
    let mut vowels = vec![0;5];
    let mut vowel_count = 0;
    let mut cons_count = 0;

    for (idx, c) in word.iter().enumerate() {
        if let Some(i) = is_vowel(*c) {
            if vowels[i] == 0 {
                vowel_count += 1;
            }
            vowels[i] += 1;
        } else {
            cons_count += 1;
        }

        while vowel_count == 5 && cons_count >= k {
            count += (word.len() - idx) as i64;
            if let Some(i) = is_vowel(word[s]) {
                if vowels[i] == 1 {
                    vowel_count -= 1;
                }
                vowels[i] -= 1;
            } else {
                cons_count -= 1;
            }
            s += 1;
        }
    }

    return count;
}
