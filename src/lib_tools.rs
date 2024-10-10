pub mod utils{
    pub fn search<'a>(content:&'a str,query:&str)->Vec<&'a str>{
        let mut ans=Vec::new();
        for line in content.lines(){
            if line.contains(query){
                ans.push(line);
            }
        }
        ans
    }
}