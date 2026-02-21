use std::collections::{HashMap};

fn main() {
    let mut list: Vec<u8> = vec![ 1, 2, 3, 2, 5, 7, 9, 4, 7, 2 ];

    median( &mut list );
    mode( &mut list );

    //Gemini solution
    // if let Some((median, mode)) = get_median_and_mode(&list) {
    //     println!("Mediana: {:.1} | Moda: {}", median, mode);
    // }
}

//My solution
fn median( list: &mut Vec<u8> ) {
    list.sort_unstable();

    let median: f32 = if list.len() % 2 == 0 {
        let half = list.len() / 2;
        ( list[ half ] + list[ half - 1 ] ) as f32 / 2.0
    } else {
        let half = list.len() / 2;
        ( list[ half ] ) as f32
    };

    println!( "The median is: {median:.1}" );
}

//My solution
fn mode( list: &mut Vec<u8> ) {
    let mut map = HashMap::new();

    for n in list {
        let count = map.entry( *n ).or_insert( 0 );
        *count += 1;
    }

    let mut max = 0;
    let mut mode = 0;

    for ( k, v ) in map {
        if max < v {
            max = v;
            mode = k;
        }
    };

    println!("The mode is: {mode}");
}

//Gemini solution
// pub fn get_median_and_mode(list: &[i32]) -> Option<(f32, i32)> {
//     if list.is_empty() {
//         return None;
//     }

// let mut sorted_list = list.to_vec();
//     sorted_list.sort_unstable();

//     let len = sorted_list.len();
//     let mid = len / 2;

//     let median = if len % 2 == 0 {
//         (sorted_list[mid - 1] + sorted_list[mid]) as f32 / 2.0
//     } else {
//         sorted_list[mid] as f32
//     };

//     let mut map = HashMap::new();
//     for &n in list {
//         *map.entry(n).or_insert(0) += 1;
//     }

//     let mode = map
//         .into_iter()
//         .max_by_key(|&(_, count)| count)
//         .map(|(val, _)| val)
//         .unwrap();

//     Some((median, mode))
// }
