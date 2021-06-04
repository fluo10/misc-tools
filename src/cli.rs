use clap::Clap;

#[derive(Clap)]
#[clap(name = "git-common-branch", version = "0.1", author= "fluo10 <fluo10.dev@fireturtle.net>")]
pub struct Opts {
  #[clap(subcommand)]
  pub subcmd: SubCommand,
}
#[derive(Clap)]
pub enum SubCommand{
  #[clap(version = "0.1", author= "fluo10 <fluo10.dev@fireturtle.net>", about = "Pull common branches", )]
  Pull(Pull),
  #[clap(version = "0.1", author= "fluo10 <fluo10.dev@fireturtle.net>", about = "Pull common branches", )]
  Push(Push),
  #[clap(version = "0.1", author= "fluo10 <fluo10.dev@fireturtle.net>", about = "Show list about common branches", )]
  Show(Show),
  #[clap(version = "0.1", author= "fluo10 <fluo10.dev@fireturtle.net>", about = "Show status about common branches", )]
  Status(Status)
}

#[derive(Clap)]
pub struct Pull {
  #[clap(short, long, parse(from_occurrences), about = "Set the level of verbosity",)]
  pub verbose: i32,
}
#[derive(Clap)]
pub struct Push {
  #[clap(short, long, parse(from_occurrences), about = "Set the level of verbosity",)]
  pub verbose: i32,
}
#[derive(Clap)]
pub struct Show {
  #[clap(short, long, parse(from_occurrences), about = "Set the level of verbosity",)]
  pub verbose: i32,
  input: String,
}
#[derive(Clap)]
pub struct Status {
  #[clap(short, long, parse(from_occurrences), about = "Set the level of verbosity",)]
  pub verbose: i32,
  input: String,
}