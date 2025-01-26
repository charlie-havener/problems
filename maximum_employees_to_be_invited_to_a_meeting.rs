pub fn maximum_invitations(favorites: Vec<i32>) -> i32 {

    let mut indegree: Vec<usize> = vec![0; favorites.len()];
    for i in 0..favorites.len() {
        indegree[favorites[i] as usize] += 1;
    }

    let mut queue: Vec<usize> = vec![];
    for (idx, ind) in indegree.iter().enumerate() {
        if *ind == 0 {
            queue.push(idx);
        }
    }

    let mut depths: Vec<usize> = vec![1; favorites.len()];
    while let Some(n) = queue.pop() {
        let nxt = favorites[n] as usize;
        depths[nxt] = depths[nxt].max(depths[n] + 1);
        indegree[nxt] -= 1;
        if indegree[nxt] == 0 { queue.push(nxt) }
    }

    let mut ans = 0;
    let mut two_cyc = 0;
    for idx in 0..favorites.len() {
        if indegree[idx] == 0 { continue }
        let mut len = 0;
        let mut curr = idx;
        while indegree[curr] > 0 {
            indegree[curr] = 0;
            len += 1;
            curr = favorites[curr] as usize;
        }
        if len == 2 {
            two_cyc += depths[idx] + depths[favorites[idx] as usize];
        } else {
            ans = ans.max(len);
        }
    }
    return ans.max(two_cyc) as i32;
}

#[test]
fn tests() {
    let favorites = vec![2,2,1,2];
    assert_eq!(3, maximum_invitations(favorites));

    let favorites = vec![1,2,0];
    assert_eq!(3, maximum_invitations(favorites));

    let favorites = vec![3,0,1,4,1];
    assert_eq!(4, maximum_invitations(favorites));

    let favorites = vec![1,0,3,2,5,6,7,4,9,8,11,10,11,12,10];
    assert_eq!(11, maximum_invitations(favorites));

}
