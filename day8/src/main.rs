fn part_one(content: &str) {
    let mut chars_code = 0x0;
    let mut chars_memory = 0x0;

    for li in content.lines() {
        let mut unesc = String::with_capacity(li.len());
        let mut chars = li.chars();
        chars_code += li.len();

        while let Some(c) = chars.next() {
            if c == '\"' {
                continue;
            }
            else if c == '\\' {
                match chars.next().unwrap() {
                    '\"' => {
                        unesc.push('\"');
                    },
                    '\\' => {
                        unesc.push('\\');
                    },
                   'x' => {
                        let mut num = String::new();
                        num.push(chars.next().unwrap());
                        num.push(chars.next().unwrap());
                        let num = u32::from_str_radix(&num, 0x10).unwrap();
                        /*

                        println!("num: {}", num);
                        let c = char::from_digit(num, 10).unwrap();
                        println!("char: {}", c);
                        */

                       // placeholder because input contains non valid ASCII codes
                       unesc.push('a');
                    },
                    _ => ()
                }
            }
            else {
                unesc.push(c);
            }
        }

        chars_memory += unesc.len();

    }
    println!("Diff1: {} ", chars_code - chars_memory);
}

fn part_two(content: &str) {
    let mut chars_orig = 0x0;
    let mut chars_esc = 0x0;

    for li in content.lines() {
        let mut esc = String::new();
        let mut chars = li.chars();
        chars_orig += li.len();

        esc.push('\"');

        while let Some(c) = chars.next() {
            if c == '\\' {
                match chars.next().unwrap() {
                    '\"' => esc.push_str("\\\\\\\""),
                    '\\' => esc.push_str("\\\\\\\\"),
                    'x' => esc.push_str("\\\\x"),
                    _    => ()
                }
            }
            else if c == '\"' {
                esc.push_str("\\\"");
            }
            else {
                esc.push(c);
            }
        }
        
        esc.push('\"');
        chars_esc += esc.len();

        println!("Orig: {}, Esc: {}", li, esc);

    }
    println!("Diff2: {} ", chars_esc - chars_orig);
}

fn main() {
    let content = include_str!("../input.txt");
    
    part_one(&content);
    part_two(&content);
}
