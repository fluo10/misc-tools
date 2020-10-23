extern crate clap;
mod cli;
use clap::Clap;
use cli::{Opts, SubCommand, Push, Pull, Show, Add, Remove,};


fn main() {
    let opts: Opts = Opts::parse();
    match opts.subcmd {
        SubCommand::Push(t) => {
            //if t.debug {
            //    println!("Printing debug info...");
            //} else {
            //    println!("Printing normally...");
            //}
            //match t.subcmd{
            //    
            //}
        }
        SubCommand::Pull(t) => {
            println!("Pull");
        }
        SubCommand::Push(t) => {
            println!("Push");
        }
        SubCommand::Add(t) => {
            println!("Add");
        }
        SubCommand::Remove(t) => {
            println!("Remove");
        }
        SubCommand::Show(t) => {
            println!("Show");
        }
    }
//    println!("Using input file: {}", opts.input);
}
