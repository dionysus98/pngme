use std::env;
mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let args = args::parse_args(env::args().collect::<Vec<String>>());
    dbg!(args);
    Ok(())
}
