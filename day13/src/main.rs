use std::collections::{HashMap, HashSet};
use regex::Regex;
use itertools::Itertools;

fn main() {
    let content = include_str!("../input.txt");
    let re: Regex = Regex::new(
        r"(.*) would (.*) (.*) happiness units by sitting next to (.*)."
        ).unwrap();

    let mut table : HashMap<String, HashMap<String, i64>> = HashMap::new();
    let mut names = HashSet::new();

    for l in content.lines() {
        let vals = re.captures(l).unwrap();

        if vals.len() == 5 {
            let p1 = vals[1].to_string();
            let p2 = vals[4].to_string();
            let mut num = vals[3].parse::<i64>().unwrap();
            if &vals[2] == "lose" {
                num *= -1;
            }

            names.insert(p1.clone());

            if table.contains_key(&p1) {
                let entry = table.get_mut(&p1).unwrap();
                entry.insert(p2, num);
            }
            else {
                let mut entry : HashMap<String, i64> = HashMap::new();
                entry.insert(p2, num);
                table.insert(p1, entry);
            }
        }
    }

    // part two
    let me : HashMap<String, i64> = HashMap::new();
    table.insert("Me".to_string(), me);
    for n in names.iter() {
        let entry = table.get_mut(n).unwrap();
        entry.insert("Me".to_string(), 0x0);

        let me = table.get_mut(&"Me".to_string()).unwrap();
        me.insert(n.to_string(), 0x0);
    }
    names.insert("Me".to_string());

    let mut max_happiness : i64 = 0x0;
    for perm in names.iter().permutations(names.len()) {
        let mut tmp : i64 = 0x0;
        for (l, p, r) in perm.iter().circular_tuple_windows() {
            let entry = table.get(p.clone()).unwrap();
            tmp += entry.get(l.clone()).unwrap();
            tmp += entry.get(r.clone()).unwrap();
        }
        
        if tmp > max_happiness {
            max_happiness = tmp;
        }
    }

    println!("Optimal seating: {}", max_happiness);
}
