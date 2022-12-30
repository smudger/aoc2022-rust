use std::{thread, time};

pub fn part1() -> String {
    "Hello, part 1!".to_string()
}

pub fn part2() -> String {
    let twenty_millis = time::Duration::from_millis(20);

    thread::sleep(twenty_millis);
    "Hello, part 2!".to_string()
}
