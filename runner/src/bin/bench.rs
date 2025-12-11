use took::{Timer, Took};

const RUNS: usize = 100;

fn main() {
    println!("Benchmarking all days with {} runs...\n", RUNS);

    let times: Vec<_> = runner::jobs()
        .iter()
        .map(|j| {
            (
                j.1,
                (0..RUNS)
                    .map(|_| {
                        let took = Timer::new();
                        j.0();
                        took.took().into_std()
                    })
                    .min()
                    .unwrap(),
            )
        })
        .collect();

    println!("\n=== Benchmark Results ===\n");
    times.iter().for_each(|t| Took::from_std(t.1).describe(t.0));
    println!();
    Took::from_std(times.into_iter().map(|(_, t)| t).sum()).describe("everything");
}
