pub fn find_the_city(n: i32, edges: Vec<Vec<i32>>, distance_threshold: i32) -> i32 {

    let n = n as usize;

    let mut dist = vec![vec![i32::MAX; n]; n];
    for i in 0..n {
        dist[i][i] = 0;
    }


    for edge in &edges {
        let (s, d, w) = (edge[0] as usize, edge[1] as usize, edge[2]);
        dist[s][d] = w;
        dist[d][s] = w;
    }

    for i in 0..n {
        for s in 0..n {
            for d in 0..n {
                let tmp = dist[s][i].checked_add(dist[i][d]).unwrap_or(i32::MAX);
                dist[s][d] = dist[s][d].min(tmp);
            }
        }
    }

    let mut ans = (0, i32::MAX);
    for r in 0..n {
        let mut count = 0;
        for c in 0..n {
            if dist[r][c] <= distance_threshold {
                count += 1;
            }
        }
        if count <= ans.1 {
            ans = (r, count);
        }
    }

    return ans.0 as i32;
}

#[test]
fn tests() {
    let n = 4;
    let e = vec![vec![0,1,3],vec![1,2,1],vec![1,3,4],vec![2,3,1]];
    let d = 4;
    assert_eq!(3, find_the_city(n,e,d));

    let n = 5;
    let e = vec![vec![0,1,2],vec![0,4,8],vec![1,2,3],vec![1,4,2],vec![2,3,1],vec![3,4,1]];
    let d = 2;
    assert_eq!(0, find_the_city(n,e,d));
}
