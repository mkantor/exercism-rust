pub fn verse(beers: u64) -> String {
    match beers {
        0 => "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string(),
        1 => "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n".to_string(),
        2 => "2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n".to_string(),
        _ => format!("{0} bottles of beer on the wall, {0} bottles of beer.\nTake one down and pass it around, {1} bottles of beer on the wall.\n", beers, beers - 1),
    }
}

pub fn sing(beers_at_start: u64, beers_at_end: u64) -> String {
    // Note that this range will not include beers_at_start.
    (beers_at_end..beers_at_start).rev().fold(
        verse(beers_at_start),
        |acc, beers| acc + "\n" + &verse(beers),
    )
}
