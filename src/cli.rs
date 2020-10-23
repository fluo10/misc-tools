use clap::Clap;

#[derive(Clap)]
#[clap(name = "dotfiles-handler", version = "0.1", author= "fluo10 <fluo10@fireturtle.net>", about = "Helper for handling zola content")]
pub struct Opts {
  #[clap(subcommand)]
  pub subcmd: SubCommand,
}
#[derive(Clap)]
pub enum SubCommand{
  #[clap(version = "0.1", author= "fluo10 <fluo10@fireturtle.net>", about = "Push config files", )]
  Push(Push),
  #[clap(version = "0.1", author= "fluo10 <fluo10@fireturtle.net>", about = "Pull config files", )]
  Pull(Pull),
  #[clap(version = "0.1", author= "fluo10 <fluo10@fireturtle.net>", about = "Add config files", )]
  Add(Add),
  #[clap(version = "0.1", author= "fluo10 <fluo10@fireturtle.net>", about = "Remove config files", )]
  Remove(Remove),
  #[clap(version = "0.1", author= "fluo10 <fluo10@fireturtle.net>", about = "Show config files", )]
  Show(Show),
}

#[derive(Clap)]
pub struct Push {
  #[clap(short, long, parse(from_occurrences), about = "Set the level of verbosity",)]
  pub verbose: i32,
}
#[derive(Clap)]
pub struct Pull {
  #[clap(short, long, parse(from_occurrences), about = "Set the level of verbosity",)]
  pub verbose: i32,
}
#[derive(Clap)]
pub struct Add {
  #[clap(short, long, parse(from_occurrences), about = "Set the level of verbosity",)]
  pub verbose: i32,
  input: String,
}
#[derive(Clap)]
pub struct Remove {
  #[clap(short, long, parse(from_occurrences), about = "Set the level of verbosity",)]
  pub verbose: i32,
  input: String,
}
#[derive(Clap)]
pub struct Show {
}