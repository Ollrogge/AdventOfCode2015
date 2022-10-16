use itertools::Itertools;

fn main() {
    let content = include_str!("../input.txt");
    let containers = content
                    .lines()
                    .map(|l| l.parse::<u32>().unwrap())
                    .collect::<Vec<_>>();

    let mut cnt = 0x0;
    let mut min_vals = vec![0; containers.len()];;
    for s in containers.into_iter().powerset() {
        let sum : u32 = s.iter().sum();

        if sum == 150 {
            min_vals[s.len()] += 1; 
            cnt += 1;
        }
    }
    
    println!("Amt combinations: {}", cnt);
    println!("Amt ways minimum num containers: {:?}",
             min_vals.into_iter().find(|&v| v > 0).unwrap());
}
