use std::collections::HashMap;

pub fn divide_players(skill: Vec<i32>) -> i64 {

    let mut chem_sum: i64 = 0;

    let mut hm: HashMap<i32, usize> = HashMap::new();

    let total: i32 = skill.iter().sum();
    let team_total = total / (skill.len() as i32 / 2);

    if team_total * (skill.len() as i32 / 2) != total { return -1 }

    for v in skill {
        if v >= team_total { return -1 }
        let comp = team_total - v;
        let x = hm.get_mut(&comp);
        if let Some(inner) = x {
            *inner -= 1;
            if *inner == 0 {
                hm.remove(&comp);
            }
            chem_sum += v as i64 * comp as i64;
        } else {
            hm.entry(v).and_modify(|c| *c += 1).or_insert(1);
        }
    }
    
    if hm.len() > 0 { return -1 }

    return chem_sum;
}

#[test]
fn tests() {
    let skill = vec![3,2,5,1,3,4];
    assert_eq!(22, divide_players(skill));

    let skill = vec![3,4];
    assert_eq!(12, divide_players(skill));

    let skill = vec![1,1,2,3];
    assert_eq!(-1, divide_players(skill));

    let skill = vec![1,5,5,2];
    assert_eq!(-1, divide_players(skill));

    let skill = vec![2,3,4,2,5,5];
    assert_eq!(32, divide_players(skill));
}
