use std::time::Duration;
use took::Timer;

pub fn jobs() -> &'static [(&'static str, fn() -> String, fn() -> String)] {
    &[
        ("01", day01::part1, day01::part2),
        // ("02", day02::part1, day02::part2),
        // ("03", day03::part1, day03::part2),
    ]
}

pub fn times(runs: usize) -> Vec<(&'static str, Duration, Duration)> {
    jobs()
        .iter()
        .map(|j| (j.0, benchmark(j.1, runs), benchmark(j.2, runs)))
        .collect()
}

fn benchmark(solution: fn() -> String, runs: usize) -> Duration {
    (0..runs)
        .map(|_| {
            let took = Timer::new();
            solution();
            took.took().into_std()
        })
        .min()
        .unwrap()
}
