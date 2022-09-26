fn main() {
    let content = include_str!("../input.txt");

    let mut floor = 0x0;
    let mut is_basement = false;

    for (i, x) in content.chars().enumerate() {
        if x == '(' {
            floor += 0x1;
        }
        else if x == ')'{
            floor -= 0x1;

            if floor < 0x0 && !is_basement{
                println!("In basement: {}", i + 1);
                is_basement = true;
            }
        }
    }

    println!("Floor: {floor}");
}

