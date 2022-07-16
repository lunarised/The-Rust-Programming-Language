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
    for (i, _x) in days.iter().enumerate() {
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

    println!("{}", fib(12));
    println!("{}", fahrenheit_to_celcius(100));
    println!("{}", celcius_to_fahrenheit(100));
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

fn fib(n: i32) -> i32 {
    let mut a = 0;
    let mut b = 1;
    let mut hold;
    for _i in 0..n {
        hold = a;
        a = b;
        b = hold + a;
    }
    return a;
}

fn fahrenheit_to_celcius(f: i32) -> i32 {
    (f64::from(f - 32) * (5.0 / 9.0)) as i32
}
fn celcius_to_fahrenheit(c: i32) -> i32 {
    (f64::from(c) * (9.0 / 5.0)) as i32 + 32
}
