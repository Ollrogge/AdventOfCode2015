use itertools::Itertools;

#[derive(Debug, Copy, Clone, PartialEq)]
enum State {
    ON,
    OFF
}

fn get_new_state(_y: usize, _x: usize, matrix: &[[State; 100]; 100]) -> State {
    let steps = vec![-1, -1, 0, 1, 1]
        .into_iter().permutations(2)
        .unique()
        .collect::<Vec<_>>();

    let mut neighbors_on = 0x0;
    for step in steps {
        let y = _y as i32 + step[0] as i32;
        let x = _x as i32 + step[1] as i32;

        if x >= 0 && (x as usize) < matrix[0].len() && y >= 0 && (y as usize) < matrix.len() {
            if matrix[y as usize][x as usize] == State::ON {
                neighbors_on += 1;
            }
        }
    }

    match matrix[_y][_x] {
        State::ON => {
            if neighbors_on == 2 || neighbors_on == 3 {
                return State::ON
            }
            else {
                return State::OFF
            }
        },
        State::OFF => {
            if neighbors_on == 3 {
                return State::ON
            }
            else {
                return State::OFF
            }
        }
    }
}

fn main() {
    let content = include_str!("../input.txt");
    let mut grid1 = [[State::OFF; 100]; 100];
    let mut grid2 = [[State::OFF; 100]; 100];

    let mut grid = &mut grid1;
    let mut tmp_grid = &mut grid2;

    for (y,l) in content.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            match c {
                '#' => grid[y][x] = State::ON,
                _ => ()
            }
        }
    }

    let corners = vec![0, 0, grid.len()-1, grid.len()-1]
        .into_iter().permutations(2)
        .unique()
        .collect::<Vec<_>>();

    for c in corners.iter() {
        let (y, x) = c.iter().collect_tuple().unwrap();
        grid[*y][*x] = State::ON;
        tmp_grid[*y][*x] = State::ON;
    }

    for _ in 0..100 {
        for y in 0..grid[0].len() {
            for x in 0..grid.len() {
                if corners.contains(&vec![y, x]) {
                    continue;
                }
                tmp_grid[y][x] = get_new_state(y, x, grid);
            }
        }

        let tmp = grid;
        grid = tmp_grid;
        tmp_grid = tmp;
    }

    let sum = grid.iter()
            .flat_map(|v| v.iter())
            .filter(|x| **x == State::ON)
            .count();

    println!("Sum: {}", sum);
}
