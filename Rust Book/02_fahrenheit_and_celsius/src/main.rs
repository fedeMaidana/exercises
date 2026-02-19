fn main() {
    let celcius_input = 28.0;
    let fahrenheit_temp = to_fahrenheit( celcius_input );
    println!("{celcius_input} degrees Celsius are {fahrenheit_temp:.1} degrees Fahrenheit");

    let fahrenheit_input = 73.0;
    let celsius_temp = to_celsius( fahrenheit_input );
    println!("{fahrenheit_input} degrees Fahrenheit is {celsius_temp:.1} degrees Celsius");
}

fn to_fahrenheit( celcius: f32 ) -> f32 {
    ( celcius * 1.8 ) + 32.0
}

fn to_celsius( fahrenheit: f32 ) -> f32 {
    ( fahrenheit - 32.0 ) / 1.8
}