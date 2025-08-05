// XMAS SONG FUNCTIONS
const XMAS_LYRICS_BASE_1: &str = "On the "; // always starts
// now text after day name below
const XMAS_LYRICS_BASE_2: &str = " day of Christmas \nMy true love sent to me";
// the lines which get added with each day
const DAYS_XMAS_LYRICS: [&str; 11] = [
    "Twelve drummers drumming",
    "Eleven pipers piping",
    "Ten lords a-leaping",
    "Nine ladies dancing",
    "Eight maids a-milking",
    "Seven swans a-swimming",
    "Six geese a-laying",
    "Five gold rings",
    "Four calling birds",
    "Three French hens",
    "Two turtle doves",
];
// first gift changes if its the only gift or at the end of the list
const PARTRIDGE_OPTIONS: [&str; 2] = [
    "A partridge in a pear tree.",
    "And a partridge in a pear tree!",
];
const NUM_DAYS: u32 = 12;
const DAY_NAMES: [&str; 12] = [
    "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth",
    "eleventh", "twelfth",
];
pub fn print_lyrics_xmas() -> String {
    let mut day_lyrics = String::new();
    for day_num in 1..NUM_DAYS + 1 {
        //  add base "on the Xth day of xmas..."
        day_lyrics = String::from(XMAS_LYRICS_BASE_1);
        day_lyrics += DAY_NAMES[(day_num - 1) as usize];
        day_lyrics += XMAS_LYRICS_BASE_2;
        day_lyrics += "\n";

        // now start adding the specifics
        if day_num == 1 {
            // if only partridge then special case
            day_lyrics += PARTRIDGE_OPTIONS[0];
        } else {
            // add non partridge gifts
            day_lyrics = add_middle_days(day_lyrics, day_num - 1);
            // and add patridge at the end
            day_lyrics += PARTRIDGE_OPTIONS[1];
        }
        day_lyrics += "\n";
        // print that day's gifts
        println!("{}", day_lyrics)
    }
    day_lyrics
}

fn add_middle_days(mut day_lyrics: String, day_num: u32) -> String {
    // loop over each gift to be added in this specific day
    for day_num_ii in (1..day_num + 1).rev() {
        // need to access the gifts in reverse order, index reverse here
        let array_el_idx: u32 = (DAYS_XMAS_LYRICS.len() as u32) - day_num_ii;
        // add the specific gift
        day_lyrics += DAYS_XMAS_LYRICS
            .iter()
            .nth(array_el_idx as usize)
            .copied()
            .unwrap();
        day_lyrics += "\n";
    }
    return day_lyrics;
}
