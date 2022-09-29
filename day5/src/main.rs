use fancy_regex::Regex;

fn part_one(content: &str) -> u32 {
    let match_vowels = Regex::new(r"(.*[aeiou]){3,}").unwrap();
    let match_twice = Regex::new(r"(.)\1{1,}").unwrap();
    let match_not_forbidden = Regex::new(r"^((?!(ab|cd|pq|xy)).)*$").unwrap();
    let mut cnt : u32 = 0x0;

    for l in content.lines() {
        if match_vowels.is_match(l).unwrap() && 
           match_twice.is_match(l).unwrap() &&
           match_not_forbidden.is_match(l).unwrap() {
               cnt += 0x1;
           }
    }

    return cnt
}

fn part_two(content: &str) -> u32 {
    let match_repeat_twice = Regex::new(r".*(\w{2}).*\1").unwrap();
    let match_repeat_between = Regex::new(r"(\w).\1").unwrap();
    let mut cnt : u32 = 0x0;

    for l in content.lines() {
        if match_repeat_twice.is_match(l).unwrap() && 
           match_repeat_between.is_match(l).unwrap() {
               cnt += 0x1;
        }
    }
    
    return cnt
}

fn main() {
    let content = include_str!("../input.txt");
    println!("part one: {}", part_one(&content));
    println!("part two: {}", part_two(&content));
}
