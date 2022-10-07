use serde_json::{Value, Map};

struct Parser {
    pub cnt: i64
}

impl Parser {
    pub fn parse(&mut self, content: &str) {
        let v: Value = serde_json::from_str(content).unwrap();
        self.parse_obj(v.as_object().unwrap());
    }

    fn parse_obj(&mut self, obj: &Map<String, Value>) {
        // not good but necessary due to struct design approach
        for (_, v) in obj {
            if v.is_string() {
                if v.as_str().unwrap() == "red" {
                    return;
                }
            }
        }

        for (_, v) in obj {
            self._parse(&v);
        }
    }

    fn parse_arr(&mut self, arr: &Vec<Value>) {
        for v in arr.iter() {
            self._parse(&v);
        }
    }

    fn _parse(&mut self, v: &Value) {
        if v.is_object() {
            self.parse_obj(v.as_object().unwrap());
        }
        else if v.is_array() {
            self.parse_arr(v.as_array().unwrap());
        }
        else if v.is_number() {
            self.cnt += v.as_i64().unwrap();
        }
    }
}

fn main() {
    let content = include_str!("../input.txt");
    let mut parser = Parser {
        cnt: 0x0
    };

    parser.parse(&content);

    println!("Cnt: {}", parser.cnt);
}
