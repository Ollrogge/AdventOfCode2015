#[derive(Clone, PartialEq, Debug)]
struct Point {
    x: i64,
    y: i64
}

fn part_one(content: &str) {
    let mut grid = Vec::<Point>::new();
    let mut p = Point {
        x: 0x0,
        y: 0x0
    };
    let mut visited = 0x1;

    grid.push(p.clone());
    
    for c in content.chars() {
        match c {
            '^'=> p.y += 1,
            'v' => p.y -= 1,
            '>' => p.x += 1,
            '<' => p.x -= 1,
            _   => ()
        }

        if !grid.contains(&p) {
            visited += 1;
            grid.push(p.clone());
        }
    }

    println!("Visited1: {visited}");
}

fn part_two(content: &str) {
    let mut grid = Vec::<Point>::new();
    let mut p_s = Point {
        x: 0x0,
        y: 0x0
    };
    let mut p_r = Point {
        x: 0x0,
        y: 0x0
    };
    let mut visited = 0x1;

    grid.push(p_s.clone());

    let mut p;
    for (i, c) in content.chars().enumerate() {
        //println!("p_s: {:?}, p_r: {:?}", p_s.clone(), p_r.clone());
        if i % 0x2 == 0x0 {
            p = &mut p_s;
        }
        else {
            p = &mut p_r;
        }

        match c {
            '^'=> p.y += 1,
            'v' => p.y -= 1,
            '>' => p.x += 1,
            '<' => p.x -= 1,
            _   => ()
        }

        if !grid.contains(&p) {
            visited += 1;
            grid.push(p.clone());
        }
    }

    println!("Visited2: {visited}");
}

fn main() {
    let content = include_str!("../input.txt");

    part_one(&content);
    part_two(&content);
}
