use itertools::Itertools;
use std::collections::HashSet;
use fancy_regex::Regex;

#[derive(Debug)]
struct Password {
    content: String
}

impl Password {
    pub fn increase(&mut self) {
        let mut chars = self.content.chars().rev().collect::<Vec<char>>();
        for (i, c) in chars.clone().iter().enumerate() {
            if *c == 'z' {
                chars[i] = 'a';
            }
            else {
                chars[i] = (*c as u8 + 0x1) as char;
                break;
            }
        }

        chars.reverse();
        self.content = String::from_iter(chars);
    }

    pub fn check(&self) -> bool {
        if self.content.contains('i') || self.content.contains('o')
            || self.content.contains('l') {
                return false
            }

        let RE: Regex = Regex::new(r"(.)\1").unwrap();
        let pairs = RE.find_iter(&self.content)
            .map(|m| m.unwrap().as_str())
            .collect::<HashSet<&str>>();

        if pairs.len() < 2 { 
            return false 
        }

        for (a, b, c) in self.content.chars().tuple_windows() {
            if b as u8 == a as u8 + 1 && c as u8 == b as u8 + 1 {
                return true
            }
        }

        false
    }
}

fn main() {
    let mut content = String::from("hxbxwxba");

    let mut pw = Password {
        content: content
    };

    for i in 0..2 {
        loop {
            pw.increase();
            if pw.check() {
                println!("Pw: {:?}", pw);
                break;
            }
        }
    }
}
