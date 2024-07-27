pub fn minimum_cost(source: String, target: String, original: Vec<char>, changed: Vec<char>, cost: Vec<i32>) -> i64 {

    let n = original.len();
    let mut distance = [[i64::MAX; 26]; 26];

    for idx in 0..n {
        let from = original[idx] as usize - b'a' as usize;
        let to = changed[idx] as usize - b'a' as usize;
        let weight = cost[idx] as i64;

        distance[from][to] = distance[from][to].min(weight);
    }
    
    for inter in 0..26 {
        for start in 0..26 {
            for end in 0..26 {
                let d = distance[start][inter].checked_add(distance[inter][end]).unwrap_or(i64::MAX);
                distance[start][end] = distance[start][end].min(d);
            }
        }
    }

    let mut ans: i64 = 0;
    let mut z = source.chars().zip(target.chars());
    while let Some((s,t)) = z.next() {
        if s == t { continue }
        let d = distance[s as usize - b'a' as usize][t as usize - b'a' as usize];
        if d == i64::MAX { return -1 }
        ans += d;
    }

    return ans;
}

#[test]
fn tests() {
    let s = String::from("abcd");
    let t = String::from("acbe");
    let o = vec!['a','b','c','c','e','d'];
    let c = vec!['b','c','b','e','b','e'];
    let v = vec![2,5,5,1,2,20];
    assert_eq!(28, minimum_cost(s,t,o,c,v));

    let s = String::from("aaaa");
    let t = String::from("bbbb");
    let o = vec!['a','c'];
    let c = vec!['c','b'];
    let v = vec![1,2];
    assert_eq!(12, minimum_cost(s,t,o,c,v));

    let s = String::from("abcd");
    let t = String::from("abce");
    let o = vec!['a'];
    let c = vec!['e'];
    let v = vec![10000];
    assert_eq!(-1, minimum_cost(s,t,o,c,v));
}
