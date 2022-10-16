use std::collections::HashMap;
use std::cmp::min;

const KEYS : [&'static str; 10] = ["children", "cats", "samoyeds", "pomeranians",
                                "akitas", "vizslas", "goldfish", "trees",
                                "cars", "perfumes"];
#[derive(Debug)]
struct Sue {
    num: usize,
    pub traits: HashMap<&'static str, Option<u32>>
}

impl Sue {
    pub fn new(num: usize) -> Sue {
        Sue {
            num: num,
            traits: HashMap::new()
        }
    }
    pub fn add_trait(&mut self, key: &'static str, val: Option<u32>) {
        self.traits.insert(key, val);
    }

    pub fn eq(&self, s: &Sue) -> bool {
        for k in KEYS {
            let a = self.traits.get(k).unwrap();
            let b = s.traits.get(k).unwrap();

            if a.is_some() && b.is_some() {
                let a = a.unwrap();
                let b = b.unwrap();
                match k {
                    "cats" | "trees" => {
                        // b needs to be fewer than a else it is definitely 
                        // greater
                        if !(b < a) {
                            return false;
                        }
                    }
                    "pomeranians" | "goldfish" => {
                        // b needs to be greater than a else it is definitely
                        // fewer
                        if !(b > a) {
                            return false
                        }
                    },
                    _ => {
                        if a != b {
                            return false
                        }
                    }
                }
            }
        }
        true
    }
}

impl PartialEq for Sue {
    fn eq(&self, other: &Self) -> bool {
        for k in KEYS {
            let a = self.traits.get(k).unwrap();
            let b = other.traits.get(k).unwrap();
            
            if a.is_some() && b.is_some() {
                if a.unwrap() != b.unwrap() {
                    return false;
                }
            }

        }
        true 
    }
}

fn main() {
    let output = include_str!("../input.txt");
    let mut sue = Sue::new(0x0);

    for l in output.lines() {
        let parts = l.split(": ").collect::<Vec<_>>();
        let key = parts[0];
        let val = parts[1].parse::<u32>();

        sue.add_trait(key, val.ok());
    }

    let mut aunts = Vec::new();

    let content = include_str!("../input2.txt");
    for (i, l) in content.lines().enumerate() {
        let mut sue = Sue::new(i+1);
        for k in KEYS.iter() {
            let tmp = l.find(k);
            let num = match tmp {
                Some(idx) => {
                    let start = idx + k.len() + 0x2;
                    let end = min(start + 0x2, l.len());
                    let num : u32 = l.get(start..end)
                                      .unwrap()
                                      .replace(',', "")
                                      .parse()
                                      .unwrap();
                    Some(num)
                },
                None => None
            };

            sue.add_trait(k, num);
        }

        aunts.push(sue);
    }

    // Part 1
    let mut candidates = Vec::new();
    // Part 2
    let mut candidates2 = Vec::new();
    for a in aunts.iter() {
        if *a == sue {
            candidates.push(a);
        }

        if a.eq(&sue) {
            candidates2.push(a);
        }
    }
    println!("candidates1: {:?}", candidates);
    println!("candidates2: {:?}", candidates2);
}
