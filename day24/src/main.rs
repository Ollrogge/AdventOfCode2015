use itertools::Itertools;

fn solve(weights: Vec<usize>, num_spaces: usize) -> usize {
    let mut quantum  = None;
    let mut k = 0x1;
    let total_weight = weights.iter().sum::<usize>();

    loop {
        quantum = weights.iter()
            .copied()
            .combinations(k)
            .filter(|x| x.iter().sum::<usize>() == total_weight / num_spaces)
            .map(|x| x.iter().fold(1, |sum, x| sum * x))
            .min();

        if quantum.is_some() {
            break;
        }

        k += 1;
    }

    quantum.unwrap()
}

fn main() {
    let weights = include_str!("../input.txt")
                    .lines()
                    .filter_map(|val| val.parse::<usize>().ok())
                    .collect::<Vec<_>>();

    println!("Comb1: {}", solve(weights.clone(), 3));
    println!("Comb2: {}", solve(weights, 4));
}
