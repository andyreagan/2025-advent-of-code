use rayon::prelude::*;

fn main() {
    println!("Running all days in parallel...\n");
    runner::jobs().par_iter().for_each(|(job, _)| {
        job();
    });
}
