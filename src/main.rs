mod recipe;
use std::path::Path;
use std::fs::File;
use std::error::Error;
use std::io::Read;
use crate::recipe::{RecipeBook, Recipe};
use crate::recipe::RecipeType::Meal;
use rand::distributions::weighted::alias_method::WeightedIndex;
use rand::prelude::*;

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
    let mut month_input = String::new();
    println!("Enter the month you would like a meal plan for: ");
    let _ = std::io::stdin().read_line(&mut month_input);
    let month: usize = match month_input.trim().to_lowercase().as_str() {
        "january" => 0,
        "february" => 1,
        "march" => 2,
        "april" => 3,
        "may" => 4,
        "june" => 5,
        "july" => 6,
        "august" => 7,
        "september" => 8,
        "october" => 9,
        "november" => 10,
        "december" => 11,
        _ => 255
    };
    println!("How many meals do you want? ");
    let mut num_input = String::new();
    let _ = std::io::stdin().read_line(&mut num_input);
    let num_meals : i32 = num_input.trim().parse().unwrap();
    let mut recipe_to_weight_map : Vec<(Recipe, f64)> = vec![];
    for mut recipe in tsr.recipes {
        if recipe.category == Meal && recipe.seasonable_percent(month) >= recipe.max_seasonable_percent() && recipe.meat == false {
            let mut months_in_season = 0;
            for i in 0..12 {
                if recipe.seasonable_percent(i) >= recipe.max_seasonable_percent() {
                    months_in_season += 1;
                }
            }
            recipe_to_weight_map.push((recipe, 1.0 / (months_in_season as f64)));
        }
    }

    let mut rng = thread_rng();
    let dist = WeightedIndex::new(recipe_to_weight_map.iter().map(|item| item.1).collect()).unwrap();
    for i in 0..num_meals {
        // 0% chance to print 'a', 30% chance to print 'b', 70% chance to print 'c'
        println!("Meal #{}:\n{}\n\n", i + 1, recipe_to_weight_map[dist.sample(&mut rng)].0);
    }

//    for mut rcp in tsr.recipes {
//        let max_seasonable = rcp.max_seasonable_percent();
//        if max_seasonable < 1.0 {
//            println!("{} only has {}% of its ingredients in season at a time", rcp.name, max_seasonable*100.0)
//        }
//    }
//    println!("{}", tsr.recipes[0]);
//    println!("Seasonable %: {}", tsr.recipes[0].seasonable_percent(&Month::December))
}