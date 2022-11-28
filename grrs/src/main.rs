#[allow(dead_code)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}



fn main() {
    println!("Hello, world!");
    let pattern = std::env::args().nth(3).expect("no pattern given");
    let path = std::env::args().nth(2).expect("no path given");
    println!("Looking for {} in {}", pattern, path);
}
