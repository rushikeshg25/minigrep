mod read_file;
mod lib_tools;
use std::env;
use read_file::read_file_mod;
use lib_tools::utils;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config=CliConfig::build(&args).unwrap_or_else(|error|{
        println!("Error with inputs {}",error);
        std::process::exit(1);    
    });

   
    let content=read_file_mod::read_file(&config.file_path);

    println!("Searching for Query {}", config.query);
    println!("Searching in FilePath {}", config.file_path);

    // let found=utils::search(&content, &config.query);

    for line in utils::search(&content, &config.query){
        println!("{}",line);
    }
    
}



struct CliConfig{
    query:String,
    file_path:String
}
impl CliConfig{
    fn build(args:&[String])->Result<CliConfig,String>{
        if args.len()<3 {
           return Err(String::from("Invalid minigrep call"));
        }
        let query=args[1].clone();
        let file_path=args[2].clone();
        Ok(CliConfig{query,file_path})
    }
}
