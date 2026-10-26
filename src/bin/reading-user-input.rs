use std::io;
fn main() {
    let mut input = String::new();
    println!("enter a number: ");
    io::stdin().read_line(&mut input);
    let earth_weight: f32 = input.trim().parse().unwrap();
    let mars_weight = calculate_mars_weight(earth_weight);
    println!("weight on mars: {mars_weight}kg");
}
fn calculate_mars_weight(w: f32) -> f32 {
    (w / 9.81) * 3.711
}
