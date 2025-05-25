mod gen_token;

// use self::gen_token::GenTokenOpts;
use self::gen_token::GenTokenOpts;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(version, author, about, long_about=None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Generating Apple Music developer token
    GenToken(GenTokenOpts),
}
