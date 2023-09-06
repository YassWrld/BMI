use std::io;

fn main() {
    println!("Please input your weight in kilo-grammes : ");
    let mut weight = String::new();
    io::stdin()
        .read_line(&mut weight)
        .expect("Failed to read weight!");

    let weight: f64 = weight
        .trim()
        .parse()
        .expect("Input a correct number please !");

    println!("Please input your height in centemeters: ");
    let mut height = String::new();
    io::stdin()
        .read_line(&mut height)
        .expect("Failed to read height!");

    let mut height: f64 = height
        .trim()
        .parse()
        .expect("Input a correct number please !");

    height = height / 100.0;

    let body_mass_index = weight / (height * height);

    println!("Height : {height}m    Weight : {weight}Kg   BMI : {body_mass_index} ");

    match body_mass_index {
        body_mass_index if body_mass_index < 18.5 => {
            println!("You're underweight !");
        }
        body_mass_index if body_mass_index < 25.0 => {
            println!("You're in the healthy weight range !");
        }
        body_mass_index if body_mass_index < 30.0 => {
            println!("You're in the overweight range !");
        }
        _ => {
            println!("You're in the obesse range !");
        }
    };
}
