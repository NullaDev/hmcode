mod byte_lib;
mod file_ops_lib;
mod packet_lib;

use std::env;

use file_ops_lib::file_operator::process_file_sample;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("This programme need proper argument number!");
    } else {
        process_file_sample(&args[1]);
        println!("{}", args[0]);
    }
}
