use std::str::Lines;
use regex::Regex;
use itertools::Itertools;
use itertools::iproduct;
use std::cmp;

#[derive(Debug)]
struct Entity {
    hit_points: i32,
    damage: i32,
    armor: i32,
    cost: i32,
}

impl Entity {
    fn new(hit_points: i32, damage: i32, armor: i32, cost: i32) -> Entity {
        Entity {
            hit_points,
            damage,
            armor,
            cost
        }
    }

    fn add_gear(&mut self, gear: &Gear) {
        self.damage += gear.damage;
        self.cost += gear.cost;
        self.armor += gear.armor;
    }

    fn fight(&mut self, opponent: &Entity) -> bool {
        // player attacks opponent
        let d1 = cmp::max(self.damage - opponent.armor, 1);
        // opponent attacks player
        let d2 = cmp::max(opponent.damage - self.armor, 1);

        let t1 = (opponent.hit_points as f32 / d1 as f32).ceil();
        let t2 = (self.hit_points as f32 / d2 as f32).ceil();

        // if equal player wins because player starts
        if t1 <= t2 {
            return true;
        }
        else {
            return false;
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum GearType {
    Weapon,
    Armor,
    Ring
}

#[derive(Debug)]
struct Gear {
    typ: GearType,
    cost: i32,
    damage: i32,
    armor: i32
}

impl Gear {
    pub fn new(typ: GearType, cost: i32, damage: i32, armor: i32) -> Gear {
        Gear {
            typ,
            cost,
            damage,
            armor
        }
    }
}

fn parse_gear(gear: &mut Vec<Gear>, it: &mut Lines<'_>, typ: GearType) {
    while let Some(l) = it.next() {
        if l.len() == 0 {
            break;
        }

        let match_nums = Regex::new(r"\d+").unwrap();
        let g : Vec<i32> = match_nums.find_iter(l)
            .filter_map(|digit| digit.as_str().parse().ok())
            .collect();

        if typ == GearType::Ring {
            gear.push(Gear::new(typ, g[1], g[2], g[3]));
        }
        else {
            gear.push(Gear::new(typ, g[0], g[1], g[2]));
        }
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let mut rings = Vec::new();
    let mut weapons = Vec::new();
    let mut armors = Vec::new();

    let mut it = input.lines();

    while let Some(l) = it.next() {
        if l.contains("Weapons:") {
            parse_gear(&mut weapons, &mut it, GearType::Weapon);
        }
        else if l.contains("Armor:") {
            parse_gear(&mut armors, &mut it, GearType::Armor);
        }
        else {
            parse_gear(&mut rings, &mut it, GearType::Ring);
        }
    }

    let boss = Entity::new(103, 9, 2, 0);

    // no armor case
    armors.push(Gear::new(GearType::Armor, 0, 0, 0));

    // all combinations of 0-2 rings
    let rings = rings.iter()
        .powerset()
        .filter(|r| r.len() <= 2)
        .collect::<Vec<_>>();

    let mut min_cost : i32 = std::i32::MAX;
    let mut max_cost : i32 = 0x0;

    for (rs, w, a) in iproduct!(rings.iter(), weapons.iter(), armors.iter()) {
        let mut player = Entity::new(100, 0, 0, 0);
        player.add_gear(&w);
        player.add_gear(&a);

        for r in rs {
            player.add_gear(&r);
        }

        if player.fight(&boss) {
            if player.cost < min_cost {
                min_cost = player.cost;
            }
        }
        else {
            if player.cost > max_cost {
                max_cost = player.cost;
            }
        }
    }

    println!("Min cost: {}", min_cost); 
    println!("Max cost: {}", max_cost); 
}
