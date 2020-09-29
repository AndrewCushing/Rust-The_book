use crate::challenges::chapter_8::strings_to_pig_latin::to_pig_latin;

pub fn run(){
    test1();
}

fn test1() {
    println!("{}", to_pig_latin("here is Some Funny text and things blah"));
}