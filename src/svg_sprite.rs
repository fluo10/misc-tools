use std::string::String;
use std::path::Path;
use std::io::{self, Write, BufReader, Read};
use std::fs::{self, File};
pub struct SvgSprite {
    content:  String
}
impl SvgSprite {
    pub fn new() -> SvgSprite{
        SvgSprite {content: String::new(),}
    }
    pub fn add_svg(&mut self, path: &Path){
        let svg_content = fs::read_to_string(path).unwrap();
        self.content += &svg_content;
    }
    pub fn export(&mut self, path: &Path){
        let mut file = File::create(path).unwrap();
        //let buf = BufReader::new(io::stdin()).lines().collect::<io::Result<Vec<String>>>()?.join("\n");
        writeln!(file, "{}", self.content);
        file.flush();
        //Ok(())
    }
}