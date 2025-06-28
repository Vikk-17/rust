use std::collections::HashMap;

fn main() {
    // create a HashMap, with the generic
    let mut scores:HashMap<String, u32> = HashMap::new();

    // insert the values
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 20);
    scores.insert(String::from("Black"), 30);
    scores.insert(String::from("Green"), 40);
  
    // overwrite the value => overwrite the value of Green with 50
    // scores.insert(String::from("Green"), 50);
    // directly to print the data inside the HashMap
    println!("{scores:?}");
    // access the value from the HashMap
    let team_name_blue = String::from("Blue");

    /*
     * get() => retunrs Option<&V>
     * If there is no value for that key it will return None
     * copied() => to get Option<i32> rather than an Option<&i32> 
     * unwrap_or() => to set zero if scores doesn't have an entry for the key
     */

    let score_blue = scores.get(&team_name_blue).copied().unwrap_or(0);

    println!("Team: {team_name_blue} with scores: {score_blue}");


    // iterate through HashMap
    for (key, value) in &scores {
        println!("{key}: {value}");
     }

}
