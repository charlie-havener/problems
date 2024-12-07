pub fn can_alice_win(a: Vec<String>, b: Vec<String>) -> bool {

    // 28 so there is a 'dummy' letter at the start and end
    let mut a_words = [None; 28];
    let mut b_words = [None; 28];

    // get the index of the last (largest) word of each letter
    for (idx, w) in a.iter().enumerate() {
        let l = w.as_bytes()[0];
        a_words[(l - b'a' + 1) as usize] = Some(idx);
    }
    for (idx, w) in b.iter().enumerate() {
        let l = w.as_bytes()[0];
        b_words[(l - b'a' + 1) as usize] = Some(idx);
    }
    
    println!("alice: {:?}\nbob: {:?}", a_words, b_words);
    
    let dummy = String::from("");

    // alice started by playing a word
    let mut start_idx = (a[0].as_bytes()[0] - b'a' + 1) as usize;
    if a_words[start_idx].is_some() && b_words[start_idx].is_none() { start_idx += 1 }
    for i in start_idx..a_words.len() {

        println!("{start_idx}");
        println!("\t{:?},{:?}", a_words[i], b_words[i]);

        match (a_words[i], b_words[i]) {
            // both have a word => continues
            (Some(_), Some(_)) => continue,

            // neither have a word => winner is whoever had largest previous
            (None, None) => {
                let prev_a = if a_words[i-1].is_some() { &a[a_words[i-1].unwrap()] } else { &dummy };
                let prev_b = if b_words[i-1].is_some() { &b[b_words[i-1].unwrap()] } else { &dummy };
                return prev_a > prev_b;
            },
            
            // only one has a word =>
            //  if they also had largest previous they win 
            //  o.w. continues
            (Some(_), None) => {
                let prev_a = if a_words[i-1].is_some() { &a[a_words[i-1].unwrap()] } else { &dummy };
                let prev_b = if b_words[i-1].is_some() { &b[b_words[i-1].unwrap()] } else { &dummy };
                if prev_b > prev_a { continue }
                return true;
            },
            (None, Some(_)) => {
                let prev_a = if a_words[i-1].is_some() { &a[a_words[i-1].unwrap()] } else { &dummy };
                let prev_b = if b_words[i-1].is_some() { &b[b_words[i-1].unwrap()] } else { &dummy };
                if prev_a > prev_b { continue }
                return false;
            }
        }
    }
    unreachable!();
}

#[test]
fn tests() {
    let a = vec![String::from("avokado"),String::from("dabar")];
    let b = vec![String::from("brazil")];
    assert_eq!(false, can_alice_win(a,b));

    let a = vec![String::from("ananas"),String::from("atlas"),String::from("banana")];
    let b = vec![String::from("albatros"),String::from("cikla"),String::from("nogomet")];
    assert_eq!(true, can_alice_win(a,b));

    let a = vec![String::from("hrvatska"),String::from("zastava")];
    let b = vec![String::from("bijeli"),String::from("galeb")];
    assert_eq!(true, can_alice_win(a,b));
}
