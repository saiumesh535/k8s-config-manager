mod cmp;
mod config;
mod errors;
mod paths;

use cmp::generate_cmp;
use errors::Error;

fn main() -> Result<(), Error> {
    generate_cmp()?;
    Ok(())
}
