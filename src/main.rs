use std::{thread, time};
use std::io::Write;
use std::process::Command;
use argh::FromArgs;


#[derive(FromArgs)]
/// Memury Card cli args
struct MCArgs {
    /// add program to startup
    #[argh(switch, short = 'i')]
    spawn: bool,
}


fn main() {
    let mcargs: MCArgs = argh::from_env();
    let mut cmd = "".to_string();
    cmd.push_str(&"start /min powershell -WindowStyle Hidden -Command \"& { ");
    cmd.push_str(std::env::current_exe().unwrap().to_str().unwrap());
    cmd.push_str(&"}\"\n");
    // let cmd = "/C echo hello\n";
    println!("{}", cmd);
    if mcargs.spawn {
        let status = Command::new("powershell").arg(&cmd).spawn().expect("failed to execute process");
    } else {
        loop {
            println!("create file");
            let mut file = std::fs::File::create(r"C:\temp\foo.txt").unwrap();
            file.write_all(b"Hello, world!").unwrap();
            thread::sleep(time::Duration::from_millis(1000));
        }
    }
}
