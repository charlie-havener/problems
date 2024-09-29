#[derive(Debug)]
struct Trie {
    letters: [Option<Box<Trie>>; 26],
    prefix_count: usize,
    word_end: usize,
}

impl Trie {

    fn new() -> Self {
        const D: Option<Box<Trie>> = None;
        return Self {
            letters: [D; 26],
            prefix_count: 1,
            word_end: 0,
        }
    }

    fn insert(&mut self, word: String) {
        let mut word = word.chars().peekable();
        let mut root = self;
        while let Some(ch) = word.next() {
            let idx = (ch as u8 - b'a') as usize;
            match &mut root.letters[idx] {
                Some(n) => {
                    n.prefix_count += 1;
                },
                None => {
                    root.letters[idx] = Some(Box::new(Trie::new()));
                },
            }
            if word.peek().is_none() {
                root.letters[idx].as_mut().unwrap().word_end += 1;
            }
            root = root.letters[idx].as_mut().unwrap();
        }
    }

    pub fn count_words_equal_to(&self, word: String) -> i32 {
        let mut word = word.chars();
        let mut ans = 0;
        let mut root = self;
        while let Some(ch) = word.next() {
            let idx = (ch as u8 - b'a') as usize;
            match &root.letters[idx] {
                Some(n) => {
                    ans = n.word_end;
                    root = n;
                },
                None => return 0,
            }
        }
        return ans as i32;
    }

    pub fn count_words_starting_with(&self, word: String) -> i32 {
        let mut word = word.chars();
        let mut ans = 0;
        let mut root = self;
        while let Some(ch) = word.next() {
            let idx = (ch as u8 - b'a') as usize;
            match &root.letters[idx] {
                Some(n) => {
                    ans = n.prefix_count;
                    root = n;
                },
                None => return 0,
            }
        }
        return ans as i32;
    }

    pub fn erase(&mut self, word: String) {
        let mut word = word.chars().peekable();
        let mut root = self;
        while let Some(ch) = word.next() {
            let idx = (ch as u8 - b'a') as usize;
            match &mut root.letters[idx] {
                Some(n) => {
                    n.prefix_count -= 1;
                }
                // erase calls guarantee the word exists
                None => unreachable!(),
            }
            if word.peek().is_none() {
                root.letters[idx].as_mut().unwrap().word_end -= 1;
            }
            root = root.letters[idx].as_mut().unwrap();
        }
    }
}

#[test]
fn test() {
    let mut t = Trie::new();
    println!("{t:?}");
    t.insert(String::from("apple"));
    println!("{t:?}");
    let c = t.count_words_equal_to(String::from("apple"));
    println!("1 = {c}");
    t.insert(String::from("apple"));
    println!("{t:?}");
    let c = t.count_words_equal_to(String::from("apple"));
    println!("2 = {c}");
    let c = t.count_words_equal_to(String::from("app"));
    println!("0 = {c}");
    let c = t.count_words_equal_to(String::from("dog"));
    println!("0 = {c}");

    let c = t.count_words_starting_with(String::from("apple"));
    println!("2 = {c}");
    let c = t.count_words_starting_with(String::from("app"));
    println!("2 = {c}");
    t.insert(String::from("app"));
    println!("{t:?}");
    let c = t.count_words_starting_with(String::from("apple"));
    println!("2 = {c}");
    let c = t.count_words_starting_with(String::from("app"));
    println!("3 = {c}");

    t.erase(String::from("apple"));
    let c = t.count_words_equal_to(String::from("apple"));
    println!("1 = {c}");
    let c = t.count_words_equal_to(String::from("app"));
    println!("1 = {c}");
    let c = t.count_words_starting_with(String::from("apple"));
    println!("1 = {c}");
    let c = t.count_words_starting_with(String::from("app"));
    println!("2 = {c}");

}
