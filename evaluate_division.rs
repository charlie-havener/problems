use std::collections::{HashMap, HashSet};


fn dfs(graph: &HashMap<String, HashMap<String, f64>>, eq: Vec<String>, acc: f64, visited: &mut HashSet<String>) -> f64 {
    visited.insert(eq[0].clone());

    let curr = graph.get(&eq[0]);
    if curr.is_none() { return -1.0 }
    let curr = curr.unwrap();
    if &eq[0] == &eq[1] { return 1.0 }

    if let Some(x) = curr.get(&eq[1]) {
        return acc * x;
    }

    for path in curr.keys() {
        if !visited.contains(path) {
            let result = dfs(
                graph, vec![path.clone(), eq[1].clone()],
                acc * curr.get(path).unwrap(), visited
                );
            if result != -1.0 { return acc }
        }
    }
    
    return acc;

}

fn calc_equation(equations: Vec<Vec<String>>, values: Vec<f64>, queries: Vec<Vec<String>>) -> Vec<f64> {
    let graph: HashMap<String, HashMap<String, f64>> = equations
        .into_iter()
        .zip(values.into_iter())
        .fold(HashMap::new(), |mut acc, (eq, val)| {
            // add numerator : { denominator : val }
            // and denominator : { numerator: 1.0/val } to the graph.
            (acc.entry(eq[0].clone()).or_default()).insert(eq[1].clone(), val);
            (acc.entry(eq[1].clone()).or_default()).insert(eq[0].clone(), 1.0/val);
            return acc;
        });

    println!("{:?}", graph);

    let mut ans: Vec<f64> = Vec::new();
    for query in queries.into_iter() {
        ans.push(dfs(&graph, query, 1.0, &mut HashSet::new()));
    }


    
    return ans; 

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mut equations: Vec<Vec<String>>;
        let mut values: Vec<f64>;
        let mut queries: Vec<Vec<String>>;

        equations = vec![vec![String::from("a"),String::from("b")],vec![String::from("b"),String::from("c")]];
        values = vec![2.0,3.0];
        queries = vec![vec![String::from("a"),String::from("c")],vec![String::from("b"),String::from("a")],vec![String::from("a"),String::from("e")],vec![String::from("a"),String::from("a")],vec![String::from("x"),String::from("x")]];
        //assert_eq!(vec![6.00000,0.50000,-1.00000,1.00000,-1.00000], calc_equation(equations, values, queries));

        equations = vec![vec![String::from("a"),String::from("b")],vec![String::from("b"),String::from("c")],vec![String::from("bc"),String::from("cd")]];
        values = vec![1.5,2.5,5.0];
        queries = vec![vec![String::from("a"),String::from("c")],vec![String::from("c"),String::from("b")],vec![String::from("bc"),String::from("cd")],vec![String::from("cd"),String::from("bc")]];
        //assert_eq!(vec![3.75000,0.40000,5.00000,0.20000], calc_equation(equations, values, queries));

        equations = vec![vec![String::from("a"),String::from("b")]];
        values = vec![0.5];
        queries = vec![vec![String::from("a"),String::from("b")],vec![String::from("b"),String::from("a")],vec![String::from("a"),String::from("c")],vec![String::from("x"),String::from("y")]];
        assert_eq!(vec![0.50000,2.00000,-1.00000,-1.00000], calc_equation(equations, values, queries));
    }
}
