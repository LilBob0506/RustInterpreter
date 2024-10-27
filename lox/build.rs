use std::io;
mod generate_A;
use generate_A::*;

fn main() -> io::Result<()> {
    generate_A(&"src".to_string())
}