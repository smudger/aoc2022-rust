use std::ops::Add;

const RUNS: usize = 100;
const EXPORT_PATH: &str = ".github/data/benchmark.csv";

use csv::Writer;
fn main() {
    println!("Benchmarking all days with {} runs...", RUNS);
    let times = runner::times(RUNS);

    println!("Exporting benchmark results to {}", EXPORT_PATH);
    let mut csv_writer = Writer::from_path("../".to_string() + EXPORT_PATH).unwrap();
    csv_writer.write_record(&["", "Part 1", "Part 2"]).unwrap();
    times.iter().for_each(|(day, part1, part2)| {
        let pretty_day = "{aoc}/1[Day ".to_string().add(day).add("]");
        let part1 = "{repo}/day".to_string()
            + day
            + "/src/lib.rs["
            + &part1.as_millis().to_string()
            + " ms]";
        let part2 = "{repo}/day".to_string()
            + day
            + "/src/lib.rs["
            + &part2.as_millis().to_string()
            + " ms]";
        csv_writer
            .write_record(&[pretty_day, part1, part2])
            .unwrap();
    });
    csv_writer.flush().unwrap();
}
