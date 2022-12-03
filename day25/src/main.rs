fn coord_to_num(r: usize, c: usize) -> usize {
    let tri_base = r + c - 1;
    let tri_count = tri_base * (tri_base + 1) / 2;
    tri_count -r + 1
}

fn num_to_code(num: usize) -> usize {
    (1..num).fold(20151125, |acc, _| (acc * 252533) % 33554393)
}
fn main() {
    let code = num_to_code(coord_to_num(2981, 3075));

    println!("Code: {}", code);
}
