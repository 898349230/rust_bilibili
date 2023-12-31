use std::env;
use std::process;
use minigrep::Config;

fn main() {
    let args  = env::args();

    let config = Config::new(args).unwrap_or_else(|err| {
        // 标准错误输出
        eprintln!("Problem parsing arguements --> {}", err);
        process::exit(1);
    });

    if let Err(e) =  minigrep::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}




