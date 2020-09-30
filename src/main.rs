use crate::files::read_token;

mod data_types;
mod variables_and_mutability;
mod loops_and_stuff;
mod challenges;
mod ownership;
mod structs_stuff;
mod if_let_control_flow;
mod collections;
mod string_stuff;
mod errors_stuff;
mod files;
mod traits_stuff;

fn main() {
    println!("The token is: {}", read_token());
}