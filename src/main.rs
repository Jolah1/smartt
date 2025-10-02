use crossterm::terminal;
use std::io;
use std::io::Read;

struct Cleanup;

impl Drop for Cleanup {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Could not disable raw mode")
    }
}
fn main() {
    terminal::enable_raw_mode().expect("Could not turn on raw mode");
    let mut buf = [0; 1];
    while io::stdin().read(&mut buf).expect("Failed to read line") == 1 && buf != [b'q'] {}
    panic!();

    //terminal::disable_raw_mode().expect("Could not turn off raw mode");
}
