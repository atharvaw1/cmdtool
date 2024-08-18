use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let file = &args[2];
    println!("Query is {query}");
    println!("File is {file}");
}
