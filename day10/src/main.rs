use itertools::Itertools;

fn lookandsay(s: &String) -> String {
    let mut newstr = String::new();
    for (key, group) in &s.chars().group_by(|x| *x) {
        let num = group.collect::<Vec<_>>().len();
        newstr.push_str(&num.to_string());
        newstr.push(key);
    }

    newstr
}

fn main() {
    let mut content = String::from("3113322113");
    
    for _ in 0..50 {
        content = lookandsay(&content);
    }
    
    println!("Length: {}", content.len());
}
