mod model;

use model::People;
use std::fs::File;
use std::io::BufReader;
use std::u64;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let file = File::open("data/person.json")?;
    let buffer = BufReader::new(file);

    let data: People = serde_json::from_reader(buffer)?;
    // println!("{:#?}", data.people.len());

    let mut active_user = 0;
    // find total active users
    for person in &data.people {
        if person.is_active == true {
            active_user += 1;
        }
    }
    println!("[Manual] Total active user: {}", active_user);
    let total = data.people
        .iter()
        .filter(|person| person.is_active)
        .count();
    println!("[Idiomatic] Total active user: {}", total);

    // returns Option<&Person>
    // return hightest salary
    if let Some(person) = data.people
        .iter()
        .max_by_key(|person| person.salary){
            println!("{}: {}", person.name, person.salary);
    };

    // return lowest salary
    if let Some(person) = data.people
        .iter()
        .min_by_key(|person| person.salary){
            println!("{}: {}", person.name, person.salary);
    };

    let mut hsalary: u64 = u64::MIN;
    let mut lsalary: u64 = u64::MAX;

    for person in data.people.iter() {
        // for hightest salary
        if hsalary < person.salary {
            hsalary = person.salary;
        }

        // for lowest salary
        if lsalary > person.salary {
            lsalary = person.salary;
        }
    }
    println!("The hightest salary is: {}", hsalary);
    println!("The lowest salary is: {}", lsalary);
    Ok(())
}
