use std::fs;
use std::path::Path;

fn main() {
    let src = Path::new("static");
    let dest = Path::new("target/release/static");

    if let Err(e) = fs::copy(src, dest) {
        eprintln!("Failed to copy static folder: {}", e);
    }
}
