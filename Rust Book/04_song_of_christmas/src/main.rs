fn main() {
    let ordinals = [ "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth" ];
    let gifts = [
        "a partridge in a pear tree",
        "two turtle doves",
        "three french hens",
        "four calling birds",
        "five gold rings",
        "six geese a laying",
        "seven swans a-swimming",
        "eight maids a-milking",
        "nine ladies dancing",
        "ten lords a-leaping",
        "eleven pipers piping",
        "twelve drummers drumming"
    ];

    song_christmas( &ordinals, &gifts );
}

//My solution
fn song_christmas( ordinals: &[&str], gifts: &[&str] ) {
    for day in 1..=12 {
        let ordinal = ordinals[ day - 1 ];

        println!( "\n\nOn the {ordinal} day of Christmas my true love sent to me " );

        for index in ( 0..day ).rev() {
            let gift_index = gifts[ index ];

            if day > 1 && index == 0 {
                print!( "and {gift_index}." )
            } else if day == 1 {
                print!( "{gift_index}." )
            } else {
                print!( "{gift_index}, " )
            }
        }
    }
}

//Gemini solution
//fn song_christmas() {
//    let days = [
//        "first", "second", "third", "fourth", "fifth", "sixth",
//        "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth",
//    ];
//
//    let gifts = [
//        "a partridge in a pear tree",
//        "two turtle doves",
//        "three french hens",
//        "four calling birds",
//        "five gold rings",
//        "six geese a-laying",
//        "seven swans a-swimming",
//        "eight maids a-milking",
//        "nine ladies dancing",
//        "ten lords a-leaping",
//        "eleven pipers piping",
//        "twelve drummers drumming",
//    ];
//
//    for (day_idx, day_name) in days.iter().enumerate() {
//        println!("On the {} day of Christmas my true love sent to me:", day_name);
//
//        for gift_idx in (0..=day_idx).rev() {
//            let gift = gifts[gift_idx];
//
//            match gift_idx {
//                0 if day_idx > 0 => println!("And {}.", gift),
//                0 => println!("{}.", gift),
//                _ => println!("{},", gift),
//            }
//        }
//
//        println!("");
//    }
//}
