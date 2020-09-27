use crate::challenges::chapter_3::fibonacci::fibonacci;

pub fn run(){
    test1();
    test2();
    test3();
    test4();
    test5();
    test6();
    test7();
    test8();
    test9();
    test10();
    println!("Looks like all tests passed for finding the nth term of the fibonacci sequence.");
}

fn test1(){
    assert_eq!(1, fibonacci(0));
}

fn test2(){
    assert_eq!(1, fibonacci(1));
}

fn test3(){
    assert_eq!(2, fibonacci(2));
}

fn test4(){
    assert_eq!(3, fibonacci(3));
}

fn test5(){
    assert_eq!(5, fibonacci(4));
}

fn test6(){
    assert_eq!(8, fibonacci(5));
}

fn test7(){
    assert_eq!(13, fibonacci(6));
}

fn test8(){
    assert_eq!(21, fibonacci(7));
}

fn test9(){
    assert_eq!(34, fibonacci(8));
}

fn test10(){
    assert_eq!(55, fibonacci(9));
}