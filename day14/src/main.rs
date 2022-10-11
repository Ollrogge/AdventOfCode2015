use regex::Regex;

#[derive(PartialEq)]
enum State {
    RESTING,
    FLYING
}

struct Reindeer {
    speed: usize,
    fly_len: usize,
    rest_len: usize,
    fly_left: usize,
    rest_left: usize,
    state: State,
    pub distance: usize,
    pub points: usize
}

impl Reindeer {
    pub fn new(speed: usize, fly_len: usize, rest_len: usize) -> Reindeer {
        Reindeer {
            speed,
            fly_len,
            rest_len,
            fly_left: fly_len,
            rest_left: rest_len,
            state: State::FLYING,
            distance: 0x0,
            points: 0x0
        }
    }

    pub fn tick(&mut self) {
        if self.state == State::FLYING {
            self.distance += self.speed;
            self.fly_left -= 1;

            if self.fly_left == 0x0 {
                self.state = State::RESTING;
                self.fly_left = self.fly_len;
            }
        }
        else {
            self.rest_left -= 0x1;
            if self.rest_left == 0x0 {
                self.state = State::FLYING;
                self.rest_left = self.rest_len;
            }
        }
    }
}

fn main() {
    let content = include_str!("../input.txt");
    let re: Regex = Regex::new(
        r"(.*) can fly (.*) km/s for (.*) seconds, but then must rest for (.*) seconds."
        ).unwrap();

    let mut deers : Vec<Reindeer> = Vec::new();
    for l in content.lines() {
        let vals = re.captures(l).unwrap();
        let speed = vals[2].parse::<usize>().unwrap();
        let fly_len = vals[3].parse::<usize>().unwrap();
        let rest_len = vals[4].parse::<usize>().unwrap();

        deers.push(Reindeer::new(speed, fly_len, rest_len));
    }

    for i in 0..2503 {
        for d in deers.iter_mut() {
            d.tick();
        }

        // part two
        let max = deers.iter().map(|d| d.distance).max().unwrap();
        deers.iter_mut().for_each(|d| {
            if d.distance == max {
                d.points += 0x1
            }
        });
    }

    let max = deers.iter().map(|d| d.distance).max().unwrap();
    println!("Max distance: {}", max);
    let max = deers.iter().map(|d| d.points).max().unwrap();
    println!("Max points: {}", max);
}
