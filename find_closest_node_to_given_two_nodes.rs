pub fn closest_meeting_node(edges: Vec<i32>, node1: i32, node2: i32) -> i32 {

    let mut ord_one = vec![-1; edges.len()];
    let mut curr = node1;
    let mut ord = 0;
    loop {
        if curr < 0 { break; }
        if ord_one[curr as usize] != -1 { break; }
        ord_one[curr as usize] = ord;
        curr = edges[curr as usize];
        ord += 1;
    }
    println!("{ord_one:?}");

    let mut ans: Option<(i32,usize)> = None;

    let mut ord_two = vec![-1; edges.len()];
    let mut curr = node2;
    let mut ord = 0;
    loop {
        if curr < 0 { break; }
        if ord_two[curr as usize] != -1 { break; }
        ord_two[curr as usize] = ord;

        if ord_one[curr as usize] >= 0 {
            // the max of the distance from node1->curr and node2->curr
            let largest = ord_one[curr as usize].max(ord_two[curr as usize]);
            ans = match ans {
                None => Some((largest, curr as usize)),
                Some((dist, idx)) => {
                    if largest > dist { ans }
                    else if largest < dist { Some((largest, curr as usize)) }
                    else {
                        if idx < curr as usize { ans }
                        else { Some((largest, curr as usize)) }
                    }
                }
            };
        }

        curr = edges[curr as usize];
        ord += 1;
    }

    if ans.is_none() { return -1 }
    return ans.unwrap().1 as i32;
}
