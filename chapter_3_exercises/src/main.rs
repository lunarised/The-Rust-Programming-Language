fn main() {
    twelve_days_of_christmas();
}

fn twelve_days_of_christmas() {
    let days = [
        "a partridge in a pear tree",
        "turtledoves",
        "french hens",
        "calling birds",
        "golden rings",
        "geese a-laying",
        "swans a-swimming",
        "maids a-milking",
        "ladies dancing",
        "lords a-leaping",
        "pipers piping",
        "drummers drumming",
    ];
    for (i, x) in days.iter().enumerate() {
        println!(
            "On the {} day of christmas, my true love gave to me:",
            number_grammar(i + 1)
        );
        for j in (0..i + 1).rev() {
            if j == 0 {
                if i > 0 {
                    print!("and ");
                }
                println!("{}", days[j]);
            } else {
                println!("{} {}", j + 1, days[j]);
            }
        }
        println!("");
    }
}

fn number_grammar(n: usize) -> String {
    let suffix = match n {
        1 => "st",
        2 => "nd",
        3 => "rd",
        4..=12 => "th",
        _ => "",
    };
    return format!("{}{}", n, suffix);
}
