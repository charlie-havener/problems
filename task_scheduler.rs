use std::collections::BinaryHeap;


pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
    // get the counts of each task
    let counts = tasks.iter().fold(vec![0;26], |mut acc, c| {
        acc[*c as usize - b'A' as usize] += 1;
        acc
    });

    // create the heap;
    let mut bh = counts.into_iter().fold(BinaryHeap::new(), |mut acc, c| {
        if c > 0 { acc.push(c); }
        acc
    });

    // schedule the tasks
    let mut curr_time = 0;
    while bh.len() > 0 {
        let mut count = 0;
        let mut popped = vec![];
        let mut cycle = n + 1;
        while cycle > 0 && bh.len() > 0 {
            let top = bh.pop().unwrap();
            if top > 1 {
                popped.push(top - 1);
            }
            cycle -= 1;
            count += 1;
        }

        for p in popped {
            bh.push(p);
        }

        curr_time += if bh.is_empty() { count } else { n + 1 }
    }

    return curr_time;
}

#[test]
fn tests() {
    let tasks = vec!['A','A','A','B','B','B'];
    let n = 2;
    assert_eq!(8, least_interval(tasks, n));
    let tasks = vec!['A','C','A','B','D','B'];
    let n = 1;
    assert_eq!(6, least_interval(tasks, n));
    let tasks = vec!['A','A','A', 'B','B','B'];
    let n = 3;
    assert_eq!(10, least_interval(tasks, n));
    let tasks = vec!['A','A','A','A','A','A','B','C','D','E','F','G'];
    let n = 1;
    assert_eq!(12, least_interval(tasks, n));

}
