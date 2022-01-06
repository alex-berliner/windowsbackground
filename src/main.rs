use std::{thread, time};
use std::io::Write;

fn main() {
    loop {
        println!("create file");
        let mut file = std::fs::File::create(r"C:\temp\foo.txt").unwrap();
        file.write_all(b"Hello, world!").unwrap();
        thread::sleep(time::Duration::from_millis(1000));
    }
}
