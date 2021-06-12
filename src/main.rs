extern crate git2;
extern crate clap;
mod cli;
mod config;
use clap::Clap;
use cli::{Opts, SubCommand,};
use git2::Repository;
use std::path::{Path,PathBuf};
use std::env;
use config::Config;

fn main() {
    let opts: Opts = Opts::parse();
    let verbose: i32 = opts.verbose;
    let current_path = env::current_dir().unwrap();
    let current_path = current_path.as_path();
    let repo: Repository = Repository::discover(&current_path).unwrap();
    
    let config = Config::new(&repo, PathBuf::from(&opts.config)); 
    println!("verbose: {}", opts.verbose);
    if verbose > 0 {
        println!("Currentpath: {}", current_path.to_str().unwrap());
        println!("CurrentRepo: {}", repo.path().to_str().unwrap());
        println!("Config path: {}", config.path.to_str().unwrap());      
    }

    match opts.subcmd {
        SubCommand::Add(t) => {
            println!("Add");
        }
        SubCommand::Remove(t) => {
            println!("Remove");
        }
        SubCommand::Pull(t) => {
            //if t.debug {
            //    println!("Printing debug info...");
            //} else {
            //    println!("Printing normally...");
            //}
            //match t.subcmd{
            //    
            //}
            println!("Pull")
        }
        
        SubCommand::Push(t) => {
            println!("Push");
        }
        SubCommand::Show(t) => {
            println!("Show");
        }
        SubCommand::Status(t) => {
            println!("Status");
        }
    }
//    println!("Using input file: {}", opts.input);
}
