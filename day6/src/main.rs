use regex::{Regex, Captures};

enum Op {
    ON,
    OFF,
    TOGGLE
}

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize
}

fn get_op(line: &str) -> Op {
    if line.contains("turn on") {
        return Op::ON;
    }
    else if line.contains("turn off") {
        return Op::OFF;
    }

    Op::TOGGLE
}

fn get_coordinates(line: &str) -> (Point, Point) {
    let re = Regex::new(r"\d+").unwrap();
    let nums : Vec<Captures> = re.captures_iter(&line).collect();
    let a = nums[0].get(0).unwrap().as_str().parse::<usize>().unwrap();
    let b = nums[1].get(0).unwrap().as_str().parse::<usize>().unwrap();
    let c = nums[2].get(0).unwrap().as_str().parse::<usize>().unwrap();
    let d = nums[3].get(0).unwrap().as_str().parse::<usize>().unwrap();

    (Point {x: a, y: b}, Point {x: c, y: d})
}

fn main() {
    let content = include_str!("../input.txt");
    let mut grid = [[0; 1000]; 1000];

    for l in content.lines() {
        let op = get_op(&l);
        let (p1, p2) = get_coordinates(&l);

        for y in p1.y..=p2.y {
            for x in p1.x..=p2.x {
                match op {
                    Op::ON => grid[y][x] += 1,
                    Op::OFF => grid[y][x] -= if grid[y][x] > 0 {1} else {0},
                    Op::TOGGLE => grid[y][x] += 2,
                    _ => ()
                }
            }
        }
    }

    let mut cnt = 0x0;
    // flatten 2d array to 1d, flat removes single nesting from collections
    for x in grid.iter().flat_map(|v| v.iter()) {
        cnt += x;
    }

    println!("Res: {cnt}");
}
