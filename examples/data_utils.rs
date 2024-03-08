use std::error::Error;

use astrorust::data_utils::data_utils::{download_tai_utc_data, file_exists, get_bsp_file};

fn main() -> Result<(), Box<dyn Error>> {
    // Download TAI-UTC data needed for UTC conversions (you will need internet connection)
    download_tai_utc_data()?;

    // Check if the file has been downloaded and exists
    let filename = "data/tai-utc.dat";
    let file_exists = file_exists(filename);
    println!("File '{}' exists: {}.", filename, file_exists);

    // Downloads a .bsp file if it does not exist or if an update is requested
    get_bsp_file("de405.bsp", false, 5.0)?;

    Ok(())
}
