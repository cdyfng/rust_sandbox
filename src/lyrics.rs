const LYRICS: [&str; 12] = [
    "Twelve drummers drumming",
    "Eleven pipers piping",
    "Ten lords a-leaping",
    "Nine ladies dancing",
    "Eight maids a-milking",
    "Seven swans a-swimming",
    "Six geese a-laying",
    "Five golden rings",
    "Four calling birds",
    "Three french hens",
    "Two turtle doves, and",
    "A partridge in a pear tree",
];

fn gen_lyrics(day: usize) -> String {
    let ordinal_suffix = match day {
        1 => "st",
        2 => "nd",
        3 => "rd",
        _ => "th",
    };

    let mut start = format!(
        "On the {}{} day of Christmas, my true love gave to me:",
        day,
        ordinal_suffix
    );

    for line in (&LYRICS).iter().skip(12 - day) {
        start.push_str("\n");
        start.push_str(line);
    }

    start
}

pub fn run() {
    for i in 1..=12 {
        println!("{}\n", gen_lyrics(i))
    }
}