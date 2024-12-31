pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {

    let lst = *days.last().unwrap() as usize;
    let mut min_costs = vec![0; lst + 1];

    let mut day_idx = 0;
    for idx in 1..=lst {
        if idx < days[day_idx] as usize {
            min_costs[idx] = min_costs[idx - 1];
        }
        else {
            day_idx += 1;
            let mut new = min_costs[idx - 1] + costs[0];
            new = new.min(min_costs[idx.checked_sub(7).unwrap_or(0)] + costs[1]);
            new = new.min(min_costs[idx.checked_sub(30).unwrap_or(0)] + costs[2]);
            min_costs[idx] = new;
        }
    }
    return *min_costs.last().unwrap();
}

#[test]
fn tests() {
    let days = vec![1,4,6,7,8,20];
    let costs = vec![2,7,15];
    assert_eq!(11, mincost_tickets(days, costs));

    let days = vec![1,2,3,4,5,6,7,8,9,10,30,31];
    let costs = vec![2,7,15]  ;
    assert_eq!(17, mincost_tickets(days, costs));

    let days = vec![1,4,6,7,8,20];
    let costs = vec![7,2,15];
    assert_eq!(6, mincost_tickets(days, costs));
}
