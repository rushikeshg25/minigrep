pub mod read_file_mod {
    use std::fs;

    pub fn read_file(path:&str)->String{
        let contents=fs::read_to_string(path).expect("Probably the filePath/File is Invalid");
        println!("{}",contents);
        contents
    }
    
}
