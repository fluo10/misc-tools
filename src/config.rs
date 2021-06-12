use std::fs::File;
use std::io::{Read, Write,BufReader, BufRead,};
use std::path::{Path,PathBuf};
use git2::Repository;


pub struct Config{
    pub path: PathBuf,
    prefixes: Vec<String>,
}
impl Config {

    pub fn new(repo: &Repository, rel_path: PathBuf) -> Config{
            let path = repo.workdir().unwrap();
            let mut path = path.to_path_buf();
            path.push(rel_path);
    
        let mut data: Vec<String> = Vec::new();
        for line in BufReader::new(File::open(&path).unwrap()).lines(){
            data.push(line.unwrap());
        };
    
        Config {
            path: path,
            prefixes: data.clone(), 
        }
    }
}