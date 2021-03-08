use std::io::{stdout, BufWriter};
use ferris_says::say;

fn main() {
    let stdout = stdout();
    let msg = String::from("Hello from Rustceans");
    let width = msg.chars().count();

    let mut write = BufWriter::new(stdout.lock());
    say(msg.as_bytes(), width, &mut write).unwrap();

}
