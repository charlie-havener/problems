pub fn alien_order(words: Vec<String>) -> String {

    if words.len() == 1 {
        let mut w = words[0].chars().collect::<Vec<char>>();
        w.sort_unstable();
        w.dedup();
        return w.iter().collect::<String>();
    }

    // only have a-z => 26 letters 
    let mut edges: Vec<Vec<char>> = vec![vec![]; 26];
    let mut seen: [bool;26] = [false; 26];

    for pair in words.windows(2) {
        let mut a = pair[0].bytes();
        let mut b = pair[1].bytes();

        loop {
            let f = a.next();
            let s = b.next();
            match (f,s) {
                // both are letters, either get an edge or continue
                (Some(l1), Some(l2)) => {
                    seen[(l1 - b'a') as usize] = true;
                    seen[(l2 - b'a') as usize] = true;
                    if l1 == l2 {
                        continue;
                    }
                    edges[(l1 - b'a') as usize].push(l2 as char);
                    break;
                },

                // both strings were the same
                (None, None) => break,

                // second was a prefix of first
                (Some(_), None) => return "".into(),

                // first was a prefix of second
                (None, Some(l)) => {
                    seen[(l - b'a') as usize] = true;
                    break;
                }
            }
        }
        // make sure all letters in each string are marked as letters in the alien dictionary
        while let Some(l) = a.next() {
            seen[(l - b'a') as usize] = true;
        }
        while let Some(l) = b.next() {
            seen[(l - b'a') as usize] = true;
        }
    }

    //println!("{edges:?}");
    //println!("{seen:?}");

    let letter_count = seen.iter().filter(|b| **b).count();
    //println!("{letter_count}");


    let mut in_degree: [i32;26] = seen.map(|b| if b { 0 } else { -1 });
    for e in &edges {
        for c in e.iter() {
            in_degree[(*c as u8 - b'a') as usize] += 1;
        }
    }
    //println!("{in_degree:?}");

    let mut queue = in_degree.iter().enumerate().filter_map(|(idx, c)| if *c == 0 { Some(idx) } else { None }).collect::<Vec<_>>();
    //println!("{queue:?}");

    let mut ans = String::new();
    while let Some(idx) = queue.pop() {
        ans.push((idx as u8 + b'a') as char);
        for nei in &edges[idx] {
            let pos = (*nei as u8 - b'a') as usize;
            in_degree[pos] -= 1;
            if in_degree[pos] == 0 {
                queue.push(pos);
            }
        }
    }
    
    //println!("ans: {ans:?}; count: {letter_count}");
    if ans.len() != letter_count { return "".into() }
    
    return ans;
}

#[test]
fn test() {
    
    let w = vec![String::from("wrt"),String::from("wrf"),String::from("er"),String::from("ett"),String::from("rftt")];
    assert_eq!(String::from("wertf"), alien_order(w));

    let w = vec![String::from("z"),String::from("x")];
    assert_eq!(String::from("zx"), alien_order(w));

    let w = vec![String::from("z"),String::from("x"),String::from("z")];
    assert_eq!(String::from(""), alien_order(w));
    
    let w = vec![String::from("ab"),String::from("ab"),String::from("abc")];
    assert_eq!(String::from("cba"), alien_order(w));

    let w = vec![String::from("abc"),String::from("ab")];
    assert_eq!(String::from(""), alien_order(w));

    let w = vec![String::from("wnlb")];
    assert_eq!(String::from("blnw"), alien_order(w));

    let w = vec![String::from("wnlb"), String::from("wnlb")];
    assert_eq!(String::from("wnlb"), alien_order(w));

    let w = vec![String::from("aba")];
    assert_eq!(String::from("ab"), alien_order(w));
}
