pub mod utils{
    pub fn search<'a>(content:&'a str,query:&str)->Vec<&'a str>{
        let query=query.to_lowercase();
        let mut ans=Vec::new();
        for line in content.lines(){
            if line.to_lowercase().contains(&query){
                ans.push(line);
            }
        }
        ans
    }
}