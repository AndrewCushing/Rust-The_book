use std::fs::File;
use std::io;
use std::io::Read;
use std::fs;

pub fn read_token() -> String {
    let open_result = File::open("C:\\Users\\andyc\\Desktop\\first_token.jwt");
    let mut file = match open_result {
        Ok(file) => file,
        Err(error) => panic!("Problem  opening the file {:?}", error)
    };
    let mut token = String::new();
    let read_result = file.read_to_string(&mut token);
    match read_result {
        Ok(_) => (),
        Err(error) => panic!("Error reading from file {:?}", error)
    };
    token
}

pub fn read_token_with_error_propagation() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("C:\\Users\\andyc\\Desktop\\first_token.jwt")?.read_to_string(&mut s)?;
    Ok(s)
}

pub fn read_token_even_shorter() -> Result<String, io::Error> {
    fs::read_to_string("C:\\Users\\andyc\\Desktop\\first_token.jwt")
}