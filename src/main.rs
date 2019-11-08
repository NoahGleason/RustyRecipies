mod recipe;
use std::path::Path;
use std::fs::File;
use std::error::Error;
use std::io::Read;
use crate::recipe::{RecipeBook, Month};
use std::cmp;

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
    for rcp in tsr.recipes {
        let mut max_seasonable : f64 = 0.0;
        for i in 0..12 {
            max_seasonable = max_seasonable.max(rcp.seasonable_percent(i))
        }
        if max_seasonable < 1.0 {
            println!("{} only has {}% of its ingredients in season at a time", rcp.name, max_seasonable*100.0)
        }
    }
//    println!("{}", tsr.recipes[0]);
//    println!("Seasonable %: {}", tsr.recipes[0].seasonable_percent(&Month::December))
}