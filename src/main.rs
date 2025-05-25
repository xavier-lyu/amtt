use amtt::{Commands, Opts, get_reader, process_gen_token};
use anyhow::Ok;
use clap::Parser;

fn main() -> anyhow::Result<()> {
    let args = Opts::parse();
    match args.cmd {
        Commands::GenToken(opts) => {
            let mut reader = get_reader(&opts.file_path)?;
            let ret = process_gen_token(&mut reader, &opts.team_id, &opts.key_id, opts.expiration)?;
            println!("{}", ret);
        }
    }

    Ok(())
}
