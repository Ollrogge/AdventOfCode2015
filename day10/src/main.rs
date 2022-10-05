fn main() {
    let mut content = String::from("3113322113");
    
    for _ in 0..50 {
        let mut chars = content.chars();
        let mut new_content = String::new();
        let mut cnt = 0x1;
        let mut last_char = chars.next().unwrap();

        while let Some(c) = chars.next() {
            if c != last_char {
                new_content.push_str(&cnt.to_string());
                new_content.push(last_char);

                cnt = 0x1;
                last_char = c;
            }
            else {
                cnt += 0x1;
            }
        }

        new_content.push_str(&cnt.to_string());
        new_content.push(last_char);

        content = new_content;
    }
    
    println!("Length: {}", content.len());
    //println!("Length: {}", content);
}
