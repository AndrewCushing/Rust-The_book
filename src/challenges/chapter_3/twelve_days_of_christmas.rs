pub fn twelve_days () {
    let gifts: [String; 12] = [
        String::from("A partridge in a pear tree"), // Another way to do the same thing it seems
        "Two turtle doves, and".to_string(),
        "Three french hens".to_string(),
        "Four calling birds".to_string(),
        "Five golden rings".to_string(),
        "Six geese a-laying".to_string(),
        "Seven swans a-singing".to_string(),
        "Eight maids a-milking".to_string(),
        "Nine ladies dancing".to_string(),
        "Ten lords a-leaping".to_string(),
        "Eleven pipers piping".to_string(),
        "12 drummers drumming".to_string()
    ];
    for num in 1usize..13usize {
        println!("On the {0} day of Christmas, my true love sent to me", num_to_position(num));
        for gift_index in (1..num+1).rev() {
            println!("{0}", gifts[gift_index-1]);
        }
        println!();
    }
}

fn num_to_position(num: usize) -> String {
    match num {
        1 => "first".to_string(),
        2 => "second".to_string(),
        3 => "third".to_string(),
        4 => "fourth".to_string(),
        5 => "fifth".to_string(),
        6 => "sixth".to_string(),
        7 => "seventh".to_string(),
        8 => "eighth".to_string(),
        9 =>  "ninth".to_string(),
        10 => "tenth".to_string(),
        11 => "eleventh".to_string(),
        12 => "twelfth".to_string(),
        default => "blah".to_string()
    }
}