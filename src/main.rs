mod recipe;
use recipe::Recipe;
use std::path::Path;
use std::fs::File;
use std::error::Error;
use std::io::Read;
use crate::recipe::{RecipeBook, Month};

fn main(){
    let path = Path::new("goodCheapEats.recipe");

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open file: {}", why.description()),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read file: {}", why.description()),
        Ok(_) => println!("Read successful"),
    }

    let tsr = RecipeBook::deserialize(s.as_str());
    println!("{}", tsr.recipes[0]);
    println!("Seasonable %: {}", tsr.recipes[0].seasonable_percent(&Month::December))
}