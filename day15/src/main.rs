use regex::Regex;
use lazy_static::lazy_static;
use itertools::Itertools;
use std::collections::HashMap;
use std::cmp::max;

lazy_static! {
    static ref RE: Regex = Regex::new(
        r"(.*): capacity (.*), durability (.*), flavor (.*), texture (.*), calories (.*)"
        ).unwrap();
}

#[derive(Debug, Clone)]
struct Ingredient {
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64,
    name: String
}

impl Ingredient {
    pub fn new() -> Ingredient {
        Ingredient {
            capacity: 0x0,
            durability: 0x0,
            flavor: 0x0,
            texture: 0x0,
            calories: 0x0,
            name: String::new()
        }
    }
}

impl From<&str> for Ingredient {
    fn from(s: &str) -> Ingredient {
        let vals = RE.captures(s).unwrap();

        Ingredient {
            name: vals[1].to_string(),
            capacity: vals[2].parse::<i64>().unwrap(),
            durability: vals[3].parse::<i64>().unwrap(),
            flavor: vals[4].parse::<i64>().unwrap(),
            texture: vals[5].parse::<i64>().unwrap(),
            calories: vals[6].parse::<i64>().unwrap(),
        }

        //let recipe = &vals[1];
    }
}

struct Cookie {
    pub recipe: Ingredient
}

impl Cookie {
    pub fn new() -> Cookie {
        Cookie {
            recipe: Ingredient::new()
        }
    }

    pub fn add(&mut self, i: Ingredient, amt: i64) {
        self.recipe.capacity += amt * i.capacity;
        self.recipe.durability += amt * i.durability;
        self.recipe.flavor += amt * i.flavor;
        self.recipe.texture += amt * i.texture;
        self.recipe.calories += amt * i.calories;
    }

    pub fn score(&self) -> i64 {
        max(self.recipe.capacity, 0x0) * max(self.recipe.durability, 0x0) 
            * max(self.recipe.flavor, 0x0) * max(self.recipe.texture, 0x0)
    }
}

fn main() {
    let content = include_str!("../input.txt");
    let mut ingredients : HashMap<String, Ingredient> = HashMap::new();

    for l in content.lines() {
        let ing : Ingredient = l.into();
        ingredients.insert(ing.name.clone(), ing);
    }

    let mut max_score = 0x0;
    let mut max_score2 = 0x0;
    for comb in ingredients.keys().combinations_with_replacement(100) {
        let mut cookie = Cookie::new();
        for (num, k) in comb.iter().dedup_with_count() {
            cookie.add(ingredients.get(&k.to_string()).unwrap().clone(), num as i64);
        }

        let score = cookie.score();

        if cookie.recipe.calories == 500 && score > max_score2 {
            max_score2 = score;
        }

        if score > max_score {
            max_score = score;
        }
    }

    println!("Max score: {}", max_score);
    println!("Max score2: {}", max_score2);
}
