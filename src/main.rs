use std::{env, vec};
mod read_file;
use read_file::read_file_mod;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config=parse_config(&args);

    let content=read_file_mod::read_file(&config.file_path);

    println!("Searching for Query {}", config.query);
    println!("Searching in FilePath {}", config.file_path);
}


struct cli_config{
    query:String,
    file_path:String
}


fn parse_config(args:&[String])->cli_config{
    let query=args[1].clone();
    let file_path=args[2].clone();
    cli_config{query,file_path}
}