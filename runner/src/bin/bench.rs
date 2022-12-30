use std::ops::Add;

use took::Took;

const RUNS: usize = 100;

fn main() {
    println!("Benchmarking all days with {} runs...", RUNS);

    let times = runner::times(RUNS);

    times.iter().for_each(|t| {
        Took::from_std(t.1).describe(&"Day ".to_string().add(t.0).add(" Part 1"));
        Took::from_std(t.2).describe(&"Day ".to_string().add(t.0).add(" Part 2"));
    });
    Took::from_std(times.into_iter().map(|(_, t1, t2)| t1 + t2).sum()).describe("Everything");
}
