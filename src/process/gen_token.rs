use crate::utils::SigningKey;
use std::io::Read;

pub fn process_gen_token(
    reader: &mut dyn Read,
    team_id: &str,
    key_id: &str,
    expiration: u64,
) -> anyhow::Result<String> {
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;

    let sk = SigningKey::try_new(&buf, key_id)?;
    let token = sk.sign(team_id, expiration)?;

    Ok(token)
}
