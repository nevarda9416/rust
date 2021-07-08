extern crate movies_lib;
use movies_lib::movies::play;
fn main() {
    println!("Inside main of test");
    play("Tutorialspoint" . to_string());
}