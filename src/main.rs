use std::{env, error::Error};

// 1. Changing main to return a Result allows us to use the '?' operator
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    // 2. Fetch the argument, converting None into an explicit error message
    let weight_str = args
        .get(1)
        .ok_or("Error: Please provide a weight as a command-line argument.")?;
    let name: &String = args.get(2).ok_or("uh oh")?;

    // 3. Parse into f32. The '?' automatically returns early if it fails.
    let weight: f32 = weight_str.parse()?;

    println!(
        "Hi {name}, your weight on mars: {:.2}kg",
        calculate_weight_on_mars(weight)
    );

    // 4. Everything went well!
    Ok(())
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}
