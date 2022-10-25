use std::collections::{HashMap, HashSet};
use std::collections::VecDeque;
use std::cmp;

fn get_distinct(replacements: &mut HashMap<&str, Vec<&str>>, molecule: &str,
                idx: usize) -> HashSet<String> {
    let max_key_len = replacements.keys().map(|l| l.len()).max().unwrap();
    let mut molecules = HashSet::new();
    for i in 0..max_key_len {
        let end = idx + i + 1;
        if end > molecule.len() {
            break;
        }
        let key = &molecule[idx..end];
        if replacements.contains_key(key) {
            for r in replacements.get(key).unwrap() {
                let tmp = format!("{}{}{}", &molecule[..idx], r, 
                                  &molecule[end..]);
                molecules.insert(tmp);
            }
        }
    }
    molecules
}

fn part1(replacements: &mut HashMap<&str, Vec<&str>>, molecule: &str) {
    let mut distinct_cnt = 0x0;
    let mut distincts = HashSet::new();
    for (i, _) in molecule.chars().enumerate() {
        distincts.extend(get_distinct(replacements, molecule, i).into_iter());
    }
    println!("Distinct molecules: {}", distincts.len());
}

fn part2(replacements: &mut HashMap<&str, Vec<&str>>, molecule: &str) {
    let mut keys = replacements.values()
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();

    // descending
    keys.sort_by(|a, b| b.len().cmp(&a.len()));

    // make a reverse map
    let mut reverse_replacement = HashMap::new();
    for k in replacements.keys() {
        for v in replacements.get(k).unwrap() {
            reverse_replacement.insert(v, k);
        }
    }

    let mut count = 0;
    let mut m = molecule.to_string();
    loop {
        for k in keys.iter() {
            if m.contains(*k) {
                let rev = reverse_replacement.get(k).unwrap();
                m = m.replacen(*k, rev, 1);
                count += 1;
                break;
            }
        }

        if m == "e" {
            break;
        }
    }

    println!("Min steps: {}", count);
}

fn main() {
    let content = include_str!("../input.txt");
    let mut iter = content.lines();
    let mut replacements : HashMap<&str, Vec<&str>> = HashMap::new();

    for l in iter.by_ref() {
        if l.len() == 0x0 {
            break;
        }
        
        let parts = l.split(" => ").collect::<Vec<_>>();
        let key = parts[0];
        let value = parts[1];

        if replacements.contains_key(key) {
            let v = replacements.get_mut(key).unwrap();
            v.push(value);
        }
        else {
            replacements.insert(key, vec![value]);
        }
    }

    let molecule = iter.next().unwrap();
    println!("Molecule: {}", molecule);

    part1(&mut replacements, molecule);
    part2(&mut replacements, molecule);
}
