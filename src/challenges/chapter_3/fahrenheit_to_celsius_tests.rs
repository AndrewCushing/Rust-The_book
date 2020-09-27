use crate::challenges::chapter_3::fahrenheit_to_celsius::convert_temp;

pub fn run () {
    test1();
    test2();
    test3();
    test4();
    println!("Looks like all tests passed for converting from Fahrenheit to Celsius.");
}

fn test1() {
    assert_eq!(20, convert_temp(68));
}

fn test2() {
    assert_eq!(-40, convert_temp(-40));
}

fn test3() {
    assert_eq!(10, convert_temp(50));
}

fn test4() {
    assert_eq!(60, convert_temp(140));
}