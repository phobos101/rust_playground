fn main() {
    println!("========================");
    println!("The 12 Days of Christmas");
    println!("========================");

    let days: [String; 12] = [
        "first".to_string(),
        "second".to_string(),
        "third".to_string(),
        "forth".to_string(),
        "fifth".to_string(),
        "six".to_string(),
        "seventh".to_string(),
        "eigth".to_string(),
        "ninth".to_string(),
        "tenth".to_string(),
        "eleventh".to_string(),
        "twelfth".to_string()
    ];

    for (idx, day) in days.iter().enumerate() {
        let main_lyric: String = get_main_lyric(&day);
        let gifts: String = get_gifts(idx);
        println!("{}\n{}\n", main_lyric, gifts);
    }
}

fn get_main_lyric(day: &str) -> String {
    let a = "On the".to_string();
    let b = day.to_string();
    let c = "day of Christmas, My true love sent to me:".to_string();
    [a, b, c].join(" ")
}

fn get_gifts(idx: usize) -> String {
    let gifts: [String; 12] = [
        "a partridge in a pear tree".to_string(),
        "Two turtle doves".to_string(),
        "Three French hens".to_string(),
        "Four calling birds".to_string(),
        "Five gold rings".to_string(),
        "Six geese a-laying".to_string(),
        "Seven swans a-swimming".to_string(),
        "Eight maids a-milking".to_string(),
        "Nine ladies dancing".to_string(),
        "Ten lords a-leaping".to_string(),
        "Eleven pipers piping".to_string(),
        "Twelve drummers drumming".to_string()
    ];

    let mut result: String = String::new().to_owned();
    let mut i = idx;
    while i > 0 {
        result.push_str(&gifts[i]);
        result.push_str("\n");
        i -= 1;
    }

    let length = &result.len();
    if length > &0 {
        result.push_str("And ");
    }

    result.push_str(&gifts[0]);

    return result;
}
