use std::path::Path;

use clap::Parser;

#[derive(Debug, Parser)]
pub struct GenTokenOpts {
    /// Apple Team ID
    #[arg(short, long = "tid", value_parser=verify_id)]
    pub team_id: String,

    /// Apple music key ID
    #[arg(short, long = "kid", value_parser=verify_id)]
    pub key_id: String,

    /// Full path to your private key file
    #[arg(short = 'p', long = "path", value_parser = verify_key_file)]
    pub file_path: String,

    /// Number of seconds until this token should expire
    #[arg(short, long = "exp", default_value_t = 2592000, value_parser= verify_expiration)]
    pub expiration: u64,
}

fn verify_id(id: &str) -> Result<String, &'static str> {
    if id.chars().count() == 10 {
        Ok(id.into())
    } else {
        Err("id shoud be 10-character")
    }
}

fn verify_key_file(filename: &str) -> Result<String, &'static str> {
    if Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("file does not exist")
    }
}

fn verify_expiration(e: &str) -> Result<u64, String> {
    let seconds: u64 = e
        .parse()
        .map_err(|_| format!("`{e}` isn't a expiration number"))?;
    if seconds <= 15777000 {
        Ok(seconds)
    } else {
        Err("expiration must not be greater than 15777000".into())
    }
}
