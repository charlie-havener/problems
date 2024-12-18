pub fn final_prices(mut prices: Vec<i32>) -> Vec<i32> {

    let mut stack: Vec<usize> = Vec::new();

    for idx in 0..prices.len() {

        while !stack.is_empty() && prices[idx] <= prices[*stack.last().unwrap()] {
            let i = stack.pop().unwrap();
            prices[i] -= prices[idx];
        }

        stack.push(idx)
    }
    
    return prices;
}

#[test]
fn tests() {
    let prices = vec![8,4,6,2,3];
    assert_eq!(vec![4,2,4,2,3], final_prices(prices));

    let prices = vec![1,2,3,4,5];
    assert_eq!(vec![1,2,3,4,5], final_prices(prices));

    let prices = vec![10,1,1,6];
    assert_eq!(vec![9,0,1,6], final_prices(prices));
}
