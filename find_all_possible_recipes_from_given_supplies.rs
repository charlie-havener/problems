use std::collections::HashMap;

pub fn find_all_recipes(recipes: Vec<String>, ingredients: Vec<Vec<String>>, supplies: Vec<String>) -> Vec<String> {

    let mut graph: HashMap<&String, (usize, bool, Vec<&String>)> = HashMap::new();
    let mut ans = vec![];

    for (idx, rec) in recipes.iter().enumerate() {

        graph.entry(rec)
            .and_modify(|v| { v.0 += ingredients[idx].len(); v.1 = true; })
            .or_insert((ingredients[idx].len(), true, vec![]));

        for ing in ingredients[idx].iter() {
            graph.entry(ing)
                .and_modify(|v| v.2.push(rec))
                .or_insert((0,false,vec![rec]));
        }

    }

    let mut queue = Vec::new();
    for ing in supplies {
        let t = graph.get(&ing);
        if t.is_none() { continue }
        if t.unwrap().0 == 0 {
            queue.push(ing);
        }
    }

    while !queue.is_empty() {
        let t = queue.pop().unwrap();
        let (_, is_rec, nei) = graph.get(&t).unwrap().clone();
        if is_rec {
            ans.push(t);
        }
        for n in nei {
            let e = graph.entry(n);
            if let std::collections::hash_map::Entry::Occupied(mut o) = e {
                o.get_mut().0 -= 1;
                if o.get().0 == 0 {
                    queue.push(n.to_string())
                }
            }
        }
    }
    
    return ans;

}


#[test]
fn test() {
    let recipes = vec![String::from("bread"),String::from("sandwich")];
    let ingredients = vec![vec![String::from("yeast"),String::from("flour")],vec![String::from("bread"),String::from("meat")]];
    let supplies = vec![String::from("yeast"),String::from("flour"),String::from("meat")];
    println!("{:?}", find_all_recipes(recipes, ingredients, supplies));
}
