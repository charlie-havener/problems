pub fn max_score_words(words: Vec<String>, letters: Vec<char>, score: Vec<i32>) -> i32 {


    fn recurse(idx: usize, letters: &mut Vec<i32>, word_scores: &Vec<i32>, words: &Vec<String>) -> i32 {
        if idx >= words.len() {
            return 0;
        }
        
        // path that skips the word
        let mut ans = recurse(idx + 1, letters, word_scores, words);

        // see if the word can be included and explore that path only if it can
        // remove the letters -> recurse (if possible) -> add the letters back
        let mut can_include = true;
        for c in words[idx].chars() {
            let idx = (c as u8 - b'a') as usize; 
            letters[idx] -= 1;
            if letters[idx] < 0 { can_include = false }
        }
        if can_include {
            ans = ans.max(word_scores[idx] + recurse(idx + 1, letters, word_scores, words));
        }
        for c in words[idx].chars() {
            let idx = (c as u8 - b'a') as usize; 
            letters[idx] += 1;
        }

        return ans;
    }

    // move the letters into an array of counts for easy/fast lookups
    let mut letters: Vec<i32> = letters.into_iter().fold(vec![0; 26], |mut acc, l| {
        let idx = (l as u8 - b'a') as usize;
        acc[idx] += 1;
        acc
    });


    // precompute the scores of each word oncea
    let word_scores = words.iter().enumerate().fold(vec![0; words.len()], |mut acc, (i, word)| {
        for c in word.chars() {
            acc[i] += score[(c as u8 - b'a') as usize];
        }
        acc
    });

    return recurse(0, &mut letters, &word_scores, &words);
}

#[test]
fn tests() {
    let words = vec![String::from("dog"), String::from("cat"), String::from("dad"), String::from("good")];
    let letters  = vec!['a','a','c','d','d','d','g','o','o'];
    let score = vec![1,0,9,5,0,0,3,0,0,0,0,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0];
    assert_eq!(23, max_score_words(words, letters, score));

    let words = vec![String::from("xxxz"),String::from("ax"),String::from("bx"),String::from("cx")];
    let letters = vec!['z','a','b','c','x','x','x'];
    let score = vec![4,4,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,5,0,10];
    assert_eq!(27, max_score_words(words, letters, score));

    let words = vec![String::from("leetcode")];
    let letters = vec!['l','e','t','c','o','d'];
    let score = vec![0,0,1,1,1,0,0,0,0,0,0,1,0,0,1,0,0,0,0,1,0,0,0,0,0,0];
    assert_eq!(0, max_score_words(words, letters, score));
}
