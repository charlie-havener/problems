use std::collections::HashSet;

pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
    
    fn to_tuple(s: String) -> (i8, i8, i8, i8) {
        let mut c = s.chars();
        (
            c.next().unwrap().to_digit(10).unwrap() as i8,
            c.next().unwrap().to_digit(10).unwrap() as i8,
            c.next().unwrap().to_digit(10).unwrap() as i8,
            c.next().unwrap().to_digit(10).unwrap() as i8
        )
    }

    


    let target = to_tuple(target);
    let mut visited = HashSet::new();
    let mut dead = HashSet::new();
    let mut ans = -1; 

    for d in deadends.into_iter() {
        dead.insert(to_tuple(d));
    }

    let start = (0,0,0,0);


    let mut queue = vec![start];
    let mut nqueue = vec![];

    while !queue.is_empty() {
        ans += 1;
        while let Some(combo) = queue.pop() {
            if dead.contains(&combo) { continue }

            if visited.contains(&combo) { continue }
            visited.insert(combo);
            
            if combo == target { return ans }
            
            let n = ((combo.0 + 1).rem_euclid(10), combo.1, combo.2, combo.3);
            if !visited.contains(&n) { nqueue.push(n) }

            let n = ((combo.0 - 1).rem_euclid(10), combo.1, combo.2, combo.3);
            if !visited.contains(&n) { nqueue.push(n) }

            let n = (combo.0, (combo.1 + 1).rem_euclid(10), combo.2, combo.3);
            if !visited.contains(&n) { nqueue.push(n) }
            
            let n = (combo.0, (combo.1 - 1).rem_euclid(10), combo.2, combo.3);
            if !visited.contains(&n) { nqueue.push(n) }

            let n = (combo.0, combo.1, (combo.2 + 1).rem_euclid(10), combo.3);
            if !visited.contains(&n) { nqueue.push(n) }

            let n = (combo.0, combo.1, (combo.2 - 1).rem_euclid(10), combo.3);
            if !visited.contains(&n) { nqueue.push(n) }

            let n = (combo.0, combo.1, combo.2, (combo.3 + 1).rem_euclid(10));
            if !visited.contains(&n) { nqueue.push(n) }

            let n = (combo.0, combo.1, combo.2, (combo.3 - 1).rem_euclid(10));
            if !visited.contains(&n) { nqueue.push(n) }
        }

        std::mem::swap(&mut queue, &mut nqueue);

    }
    return -1;
}

#[test]
fn tests() {
    let deadends = vec![String::from("8888")];
    let target = String::from("0000");
    assert_eq!(0, open_lock(deadends, target));
    
    let deadends = vec![String::from("0201"),String::from("0101"),String::from("0102"),String::from("1212"),String::from("2002")];
    let target = String::from("0202");
    assert_eq!(6, open_lock(deadends, target));

    let deadends = vec![String::from("8888")];
    let target = String::from("0009");
    assert_eq!(1, open_lock(deadends, target));

    let deadends = vec![String::from("8887"),String::from("8889"),String::from("8878"),String::from("8898"),String::from("8788"),String::from("8988"),String::from("7888"),String::from("9888")];
    let target = String::from("8888");
    assert_eq!(-1, open_lock(deadends, target));
}
