use std::error::Error;

use super::libraries;

pub fn download() -> Result<(), Box<dyn Error>> {
    libraries::get_wpimath()?;
    Ok(())
}
