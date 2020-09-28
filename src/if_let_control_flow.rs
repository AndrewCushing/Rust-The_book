pub fn things(){
    //Only do something if the value is Some(3)
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    // A shorter way to do the match part:
    if let Some(3) = some_u8_value {
        println!("three");
    }


}