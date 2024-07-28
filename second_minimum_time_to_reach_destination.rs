use std::collections::VecDeque;

pub fn second_minimum(n: i32, edges: Vec<Vec<i32>>, time: i32, change: i32) -> i32 {

    let n = n as usize;
    let mut graph = (0..=n).fold(Vec::with_capacity(n), |mut acc, _| { acc.push(Vec::<usize>::new()); acc });
    for e in edges {
        let a = e[0] as usize;
        let b = e[1] as usize;
        graph[a].push(b);
        graph[b].push(a);
    }

    let mut first = vec![-1; n + 1];
    let mut second = vec![-1; n + 1];
    first[1] = 0;

    let mut queue: VecDeque<(usize, i32)> = VecDeque::new();
    queue.push_back((1,0));


    let calc_ans = |edge_count: i32| -> i32 {
        let mut t = 0;
        for _ in 0..edge_count {
            if (t/change) % 2 == 1 {
                t = change * (t/change + 1) + time;
            } else {
                t += time;
            }
        }
        return t;
    };


    loop {
        let (node, edges) = queue.pop_front().unwrap();
        for nei in graph[node].iter() {
            if first[*nei] == -1 {
                first[*nei] = edges + 1;
                queue.push_back((*nei, edges + 1));
            } 
            else if second[*nei] == -1 && first[*nei] != edges + 1 {
                second[*nei] = edges + 1;
                if *nei == n { return calc_ans(edges + 1); }
                queue.push_back((*nei, edges + 1));
            }
        }
    }
}

#[test]
fn tests() {
    let n = 5;
    let e = vec![vec![1,2],vec![1,3],vec![1,4],vec![3,4],vec![4,5]];
    let t = 3;
    let c = 5;
    assert_eq!(13, second_minimum(n,e,t,c));

    let n = 2;
    let e = vec![vec![1,2]];
    let t = 3;
    let c = 2;
    assert_eq!(11, second_minimum(n,e,t,c));
}
