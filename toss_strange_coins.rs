pub fn probability_of_heads(prob: Vec<f64>, target: i32) -> f64 {

    let mut p = vec![0.0; prob.len()];
    p[0] = 1.0 - prob[0];
    for idx in 1..prob.len() {
        p[idx] = (1.0 - prob[idx]) * p[idx - 1];
    }
    
    let mut c = vec![0.0; prob.len()];
    c[0] = prob[0];

    if target == 0 {
        return *p.last().unwrap();
    }
    
    let mut heads = 1;
    loop {
        
        if heads == 1 {
            c[0] = prob[0];
        } else {
            c[heads - 1] = prob[heads - 1] * p[heads - 2];
        }

        for idx in heads..prob.len() {
            c[idx] = (1.0 - prob[idx]) * c[idx - 1] + prob[idx] * p[idx - 1];
        }

        if heads == target as usize {
            return *c.last().unwrap();
        }
        std::mem::swap(&mut p, &mut c);
        heads += 1;
    }
}

#[test]
fn tests() {
    let p = vec![0.4];
    let t = 1;
    println!("{}", probability_of_heads(p,t));

    let p = vec![0.5,0.5,0.5,0.5,0.5];
    let t = 0;
    println!("{}", probability_of_heads(p,t));

    let p = vec![0.5,0.5,0.5,0.5,0.5];
    let t = 1;
    println!("{}", probability_of_heads(p,t));

    let p = vec![0.5,0.5,0.5,0.5,0.5];
    let t = 2;
    println!("{}", probability_of_heads(p,t));

    let p = vec![0.5,0.5,0.5,0.5,0.5];
    let t = 3;
    println!("{}", probability_of_heads(p,t));

    let p = vec![0.5,0.5,0.5,0.5,0.5];
    let t = 4;
    println!("{}", probability_of_heads(p,t));

    let p = vec![0.5,0.5,0.5,0.5,0.5];
    let t = 5;
    println!("{}", probability_of_heads(p,t));
}
