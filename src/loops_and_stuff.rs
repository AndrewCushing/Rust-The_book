pub fn looping () {
    for num in (1..4).rev() {
        println!("Current number: {0}", num);
    }

    println!("I escaped!!!");
}