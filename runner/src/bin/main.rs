fn main() {
    println!("Running all days sequentially...\n");
    for (job, name) in runner::jobs() {
        println!("=== {} ===", name);
        job();
        println!();
    }
}
