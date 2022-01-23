use std::thread;
use std::time::Duration;

fn main() {
    let mut count = 0;
    loop {
        count += 1;
        println!("count: {}", count);
        thread::sleep(Duration::from_secs(1))
    }
}
