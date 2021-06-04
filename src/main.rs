extern crate git2;
extern crate clap;
mod cli;
use clap::Clap;
use cli::{Opts, SubCommand, Pull, Push, Show, Status,};


fn main() {
    let opts: Opts = Opts::parse();
    match opts.subcmd {
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
