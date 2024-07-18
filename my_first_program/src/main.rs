const FREEZING_POINT: f64 = 32.0;

fn fahrenheit_to_celsius(temperature: f64) -> f64 {
    (temperature - FREEZING_POINT) * (5.0 / 9.0)
}

fn main() {
    let mut fahrenheit: f64 = 100.0;

    let in_celsius = fahrenheit_to_celsius(fahrenheit);
    println!("{}°F is {:.2}°C", fahrenheit, in_celsius);

    // Using your specified code structure
    let in_celsius: f64 = fahrenheit_to_celsius(fahrenheit);

    let temperatures: [i32; 5] = [33, 34, 35, 36, 37];

    println!("{}", in_celsius);

    for &temp in temperatures.iter() {
        let in_celsius = fahrenheit_to_celsius(temp as f64);
        println!("{}°F is {:.2}°C", temp, in_celsius);
    }
}
