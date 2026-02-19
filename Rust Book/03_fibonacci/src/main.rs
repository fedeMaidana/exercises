fn main() {
    let result = fibonacci( 7 );
    println!("The result is {result}");
}

//My solution:
fn fibonacci( mut n: u32 ) -> u32 {
    let mut num_a: u32 = 0;
    let mut num_b: u32 = 1;
    let mut result: u32 = 0;

    while n > 1 {
        result = num_a + num_b;

        num_a = num_b;
        num_b = result;

        n -= 1;
    }

    result
}

// Gemini solution:
// fn fibonacci( mut n: u32 ) -> u32 {
//     if n == 0 { return 0 }

//     let ( mut a, mut b ) = ( 0, 1 );

//     while n > 1 {
//         ( a, b ) = ( b, a + b );

//         n -= 1;
//     }

//     b
// }