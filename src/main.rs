use std::env;
// use std::fs::{self, r;
use std::path::PathBuf;


mod category;
mod organize;
// use crate::category::get_category;
use crate::organize::organising;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <target_directory>", args[0]);
        return;
    }

    let target_dir = PathBuf::from(&args[1]);

    if !target_dir.exists() || !target_dir.is_dir() {
        eprintln!("The specified path is not a valid directory.");
        return;
    }
    organising(&target_dir);
}



