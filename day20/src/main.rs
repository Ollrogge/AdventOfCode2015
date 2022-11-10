fn get_present_amt(house_num: i32, part_two: bool) -> i32 {
    (1..=((house_num as f64).sqrt() as i32)).fold(0, |sum, val| {
        let mut res = sum;
        if house_num % val == 0 {
            if part_two {
                if house_num / val <= 50 {
                    res += 11 * val;
                }
            }
            else {
                res += 10 * val;
            }

            let d = house_num / val;
            if d != val {
                if part_two {
                    if house_num / d <= 50 {
                        res += 11 * d;
                    }
                }
                else {
                    res += 10 * d;
                }
            }
        }
        res
    })
}

fn main() {
    let end = 29000000;
    let mut idx = 0x0;

    while get_present_amt(idx, false) < end {
        idx += 1;
    }

    println!("Min house num part 1: {}", idx);

    idx = 0x0;
    while get_present_amt(idx, true) < end {
        idx += 1;
    }

    println!("Min house num part 2: {}", idx);

}
