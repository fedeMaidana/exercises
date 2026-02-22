fn main() {
    //My solution
    let pig_latin = to_piglatin();
    println!( "{pig_latin}" );

    //Gemini solution
    // let word1 = "chau";
    // let word2 = "ñandú";
    // let word3 = "Auto";

    // println!("{} -> {}", word1, to_pig_latin(word1));
    // println!("{} -> {}", word2, to_pig_latin(word2));
    // println!("{} -> {}", word3, to_pig_latin(word3));
}

//My solution
fn to_piglatin() -> String {
    let mut word: String = String::from( "chau" );
    let consonant = word.chars().nth( 0 );

    if let Some( c ) = consonant {
        if c != 'a' && c != 'e' && c != 'i' && c != 'o' && c != 'u' {
            word.remove( 0 );
            word.push( c );
            word.push_str( "ay" );
        } else {
            word.push_str( "hay" );
        }
    }

    word
}

//Gemini solution
// pub fn to_pig_latin(word: &str) -> String {
//     let mut chars = word.chars();

//     let first_char = match chars.next() {
//         Some(c) => c,
//         None => return String::new(),
//     };

//     match first_char.to_ascii_lowercase() {
//         'a' | 'e' | 'i' | 'o' | 'u' => format!("{word}hay"),
//         _ => {
//             let slice_start = first_char.len_utf8();
//             let rest_of_word = &word[slice_start..];

//             format!("{rest_of_word}{first_char}ay")
//         }
//     }
// }