use md5;
use num_cpus;
use std::thread;
use std::sync::{Arc, Mutex};

const INPUT : &str = "yzbqklnj";

fn brute_md5(start: usize, finish: usize) -> bool {
    for i in start..finish {
        let digest = md5::compute(format!("{INPUT}{i}"));
        let digest = format!("{:x}", digest);

        if digest.starts_with("000000") {
            println!("Digest: {}, Number: {}", digest, i);
            return true;
        }
    }

    return false
}

// could have used thread pool as well
fn main() {
    let found = Arc::new(Mutex::new(false));
    let cpus = num_cpus::get();

    while !*(found.lock().unwrap()) {
        let mut handles = Vec::new();

        for i in  0..cpus {
            let found = Arc::clone(&found);
            let h = thread::spawn(move || {
                let ret = brute_md5(i * 1_000_000, (i+1) * 1_000_000);
                if ret {
                    let mut f = found.lock().unwrap();
                    *f = true;
                }
            });
            handles.push(h);
        }

        for h in handles {
            h.join().unwrap();
        }
    }
}

