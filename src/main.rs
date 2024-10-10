use std::env;
mod read_file;
use read_file::read_file_mod::read_file;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let file_path = &args[2];

    let content=read_file(&file_path);

    println!("Searching for Query {}", query);
    println!("Searching in FilePath {}", file_path);
}
