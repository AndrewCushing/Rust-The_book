pub fn vectors(){
    // Creation:
    let integers: Vec<i32> = Vec::new();

    let mut more_integers = vec![32, 23, 24];

    // Updating:
    more_integers.push(55);
    more_integers.push(77);

    let mut v = vec![1, 2, 3, 4, 5];
    let mut y = vec![1, 2, 3, 4, 5];

    let mut v_first = v[0]; // Temporary mutable borrow but essentially a shallow copy, as v_first will be on the stack
    let mut y_first = &mut y[0]; // Mutable borrow

    //v.push(6); // Can't do this as there can only be one mutable borrow in scope at a time

    v_first = 32; // This won't actually change the value of anything in v, because we copied rather than referencing.
    *y_first = 45; // Need to dereference because y_first is a reference to part of y.

    println!("The value of v_first is: {}", v_first);
    println!("The vector v is: {:?}", v);

    println!("The value of y_first is: {}", y_first);
    println!("The vector v is: {:?}", y);

}// As vectors are structs, they're cleaned up when they go out of scope.

fn iterating() {
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
}