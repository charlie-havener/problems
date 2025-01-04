#[derive(Clone, Copy)]
struct Occurance {
    first_idx: Option<usize>,
    first_counts: [i32; 26],
    last_idx: Option<usize>,
    last_counts: [i32; 26],
}

impl Occurance {
    fn new() -> Self {
        Self {
            first_idx: None,
            first_counts: [0; 26],
            last_idx: None,
            last_counts: [0; 26],
        }
    }

    fn calculate_palindrome_count(&self, letter_idx: usize) -> i32 {
        if self.first_idx.is_none() || self.last_idx.is_none() {
            return 0;
        }
        // unwrap is safe since both are guarenteed to be Some(_)
        else if self.last_idx.unwrap() - self.first_idx.unwrap() < 2 {
            return 0;
        }
        else {
            let mut count = 0;
            for idx in 0..26 {
                let mut diff = self.last_counts[idx] - self.first_counts[idx];
                if idx == letter_idx { diff -= 1 }
                if diff > 0 {
                    count += 1;
                }
            }
            return count;
        }
    }

    fn update(&mut self, idx: usize, counts: &[i32; 26]) -> () {
        if self.first_idx.is_none() {
            self.first_idx = Some(idx);
            for i in 0..26 {
                self.first_counts[i] = counts[i];
            }
        }
        self.last_idx = Some(idx);
        for i in 0..26 {
            self.last_counts[i] = counts[i];
        }

    }
}

pub fn count_palindromic_subsequence(s: String) -> i32 {
    
    let mut ans = 0;
    let mut occ = [Occurance::new(); 26];
    let mut counts = [0; 26];

    for (idx, val) in s.bytes().enumerate() {
        let char_idx = (val - b'a') as usize;
        counts[char_idx] += 1;
        occ[char_idx].update(idx, &counts);
    }

    for (idx, o) in occ.iter().enumerate() {
        ans += o.calculate_palindrome_count(idx);
    }

    return ans;
}

#[test]
fn tests() {
    let s = String::from("aabca");
    assert_eq!(3, count_palindromic_subsequence(s));

    let s = String::from("adc");
    assert_eq!(0, count_palindromic_subsequence(s));

    let s = String::from("bbcbaba");
    assert_eq!(4, count_palindromic_subsequence(s));

    let s = String::from("aaaa");
    assert_eq!(1, count_palindromic_subsequence(s));
}
