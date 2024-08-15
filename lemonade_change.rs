pub fn lemonade_change(bills: Vec<i32>) -> bool {
    
    let mut change_counts = vec![0;3];
    for b in bills {
        match b {
            5 => change_counts[0] += 1,
            10 => {
                change_counts[1] += 1;

                // only way to make change for a 10 is with a 5
                if change_counts[0] > 0 {
                    change_counts[0] -= 1;
                    continue;
                }
                // can't make change
                return false;
            },
            20 => {
                change_counts[2] += 1;

                // try to make change with a 10 and 5 first
                if change_counts[0] > 0 && change_counts[1] > 0 {
                    change_counts[0] -= 1;
                    change_counts[1] -= 1;
                    continue;
                }
                // else try with 3 5s
                if change_counts[0] >= 3 {
                    change_counts[0] -= 3;
                    continue;
                }
                // can't make change
                return false;
                
            },
            _ => unreachable!()
        }
    }
    return true;
}


#[test]
fn tests() {
    let bills = vec![5,5,5,10,20];
    assert_eq!(true, lemonade_change(bills));
    
    let bills = vec![5,5,10,10,20];
    assert_eq!(false, lemonade_change(bills));

    let bills = vec![20,10,5,5,5];
    assert_eq!(false, lemonade_change(bills));

    let bills = vec![5,5,5,5,5,5,5,5];
    assert_eq!(true, lemonade_change(bills));

    let bills = vec![5,5,5,20];
    assert_eq!(true, lemonade_change(bills));

    let bills = vec![5,5,10,20];
    assert_eq!(true, lemonade_change(bills));
}
