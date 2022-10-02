use std::collections::{HashMap, VecDeque};
use std::error::Error;

// a -> b
fn assign<'a>(a: &str, b: &'a str, vars: &mut HashMap<&'a str, u32>) -> Option<()> {
    let mut ret = true;
    // a is a variable
    if vars.contains_key(a) {
        let tmp = vars.get(a)?;
        vars.insert(b, *tmp);
    }
    // a is a number
    else {
        println!("Number: {} {}", a, b);
        let tmp = a.parse::<u32>().ok()?;
        vars.insert(b, tmp);
    }

    Some(())
}

// NOT a -> b
fn not<'a>(a: &str, b: &'a str, vars: &mut HashMap<&'a str, u32>) -> Option<()> { 
    let tmp : u32;

    if vars.contains_key(a) {
        tmp = *vars.get(a)?;
        vars.insert(b, !tmp);
    }
    else {
        tmp = a.parse::<u32>().ok()?;
        vars.insert(b, !tmp);
    }

    Some(())
}

// a OP b -> c
fn op<'a>(op: &str, a: &str, b: &str, c: &'a str, vars: &mut HashMap<&'a str, u32>) -> Option<()> {
    let mut a_p : u32;
    let mut b_p : u32;
    let mut tmp : u32 = 0x0;
    
    if vars.contains_key(a) {
        a_p = *vars.get(a)?;
    }
    else {
        a_p = a.parse::<u32>().ok()?;
    }

    if vars.contains_key(b) {
        b_p = *vars.get(b)?;
    }
    else {
        b_p = b.parse::<u32>().ok()?;
    }

    match op {
        "AND" => {
            println!("{} AND {}", a_p, b_p);
            tmp = a_p & b_p;
        },
        "OR" => {
            println!("{} NOT {}", a_p, b_p);
            tmp = a_p | b_p; 
        },
        "LSHIFT" => {
            println!("{} LSHIFT {}", a_p, b_p);
            tmp = a_p << b_p;
        },
        "RSHIFT" => {
            println!("{} RSHIFT {}", a_p, b_p);
            tmp = a_p >> b_p;
        },
        _ => {
            println!("Unknown op: {}", op);
        }
    }

    vars.insert(c, tmp);

    Some(())
}

fn main() {
    let content = include_str!("../input.txt");
    let mut vars = HashMap::new();
    let mut ops: VecDeque<&str> = VecDeque::new();

    for l in content.lines() {
        if l.len() == 0x0 {
            continue;
        }
        ops.push_back(l);
    }

    loop {
        let mut l = ops.pop_front();
        let mut ret = None;

        let l = match l {
            Some(x) => x,
            None => break
        };

        let parts : Vec<&str> = l.split(" ").collect();

        match parts.len() {
            3 => {
                println!("Assign: {} -> {}", parts[0], parts[2]); 
                ret = assign(parts[0], parts[2], &mut vars);
            },
            4 => {
                println!("NOT: {} -> {}", parts[1], parts[3]);
                ret = not(parts[1], parts[3], &mut vars);
            },
            5 => {
                ret = op(parts[1], parts[0], parts[2], parts[4], &mut vars);
            },
            _ => {
                println!("Unknown op: {}", l);
            }
        }

        if ret.is_none() {
            ops.push_back(l);
        }
    }

    println!("Test: {}", vars.get("a").unwrap());
}
