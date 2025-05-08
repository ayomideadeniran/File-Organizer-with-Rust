use std::env; // this line is needed so that i can get command line argument.

use std::path::PathBuf; 

mod organizer;

fn main() {
    let args: Vec<String> = env::args().collect(); // i use this to locate and collect inputed cli

    if args.len() < 2 {
        eprintln!("Usage: {} <target_directory>", args[0]); // use this to make sure i can passing a file path
        return;
        // panic!("Usage: {} <target_directory>", args[0]); // this is used to make sure i can pass a file path
    }

    let target_dir = PathBuf::from(&args[1]); // take referesence to args then convert it to string
    organizer::organize(&target_dir); 
}


// In your cli run this code:  cargo run -- "/home/knights/Documents/Playing"

