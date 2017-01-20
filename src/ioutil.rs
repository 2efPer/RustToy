use std::fs::File;
use std::error::Error;
use std::io::prelude::*;


pub fn show_file(pathname:String){
    let mut file = File::open(pathname).unwrap();
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}",why.description()),
        Ok(_) => print!("contains:\n{}", s),
    }
}


