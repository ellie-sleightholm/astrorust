use std::fs::File;
use std::path::Path;

use reqwest;
use std::error::Error;
use std::io::{self, Write};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

/// Downloads the TAI-UTC data file from the U.S. Naval Observatory website.
///
/// This function makes an HTTP GET request to the U.S. Naval Observatory's website
/// to retrieve the TAI-UTC data file and saves it to a local file named "tai-utc.dat" in
/// the "data" directory.
///
/// # Errors
///
/// This function returns a `Result<(), Box<dyn Error>>`:
///
/// - If the HTTP request to retrieve the data fails.
/// - If the HTTP response body is invalid or cannot be read as text.
/// - If the local file "data/tai-utc.dat" cannot be created or written to.
///
/// # Returns
///
/// If the data is successfully downloaded and saved, it returns `Ok(())`.
pub fn download_tai_utc_data() -> Result<(), Box<dyn Error>> {
    // Get response from U.S. Naval Observatory's website
    let resp = reqwest::blocking::get("https://maia.usno.navy.mil/ser7/tai-utc.dat")?;

    // Get the response text
    let body = resp.text()?;
    // Create file in the location specified
    let mut out = File::create("data/tai-utc.dat")?;
    // Copies the entire contents of a reader into a writer.
    io::copy(&mut body.as_bytes(), &mut out)?;

    Ok(())
}

/// Checks whether a file exists.
///
/// ## Arguments
/// * `filename`: The name of the file you're checking.
///
/// ## Returns
/// A boolean corresponding to whether the file exists or not.
pub fn file_exists(filename: &str) -> bool {
    Path::new(&filename).exists()
}

/// Checks if the TAI-UTC data file exists and downloads it if necessary.
///
/// # Arguments
///
/// * `update_file` - If `true`, force an update even if the file already exists.
///
/// # Errors
///
/// This function returns a `Result<(), Box<dyn Error>>`:
///
/// - If the file already exists but `update_file` is `true`, and the update fails.
/// - If any other error occurs while checking the file or downloading it.
///
/// # Returns
///
/// If the file is already up-to-date or successfully updated, it returns `Ok(())`.
pub fn get_tai_utc_data(update_file: bool) -> Result<(), Box<dyn Error>> {
    // First check if the file already exists
    let file_exists = file_exists("data/tai-utc.dat");

    // If the file does not exist or if update is requested, download it
    if !file_exists || update_file {
        download_tai_utc_data()?;
    }

    Ok(())
}

/// Downloads a BSP (Binary Space Partitioning) file if it does not exist or if an update is requested.
///
/// # Arguments
///
/// * `file_name` - The name of the BSP file to download.
/// * `update_file` - If `true`, force an update even if the file already exists.
/// * `minutes` - The maximum number of minutes to allow for the download operation.
///
/// # Panics
///
/// This function may panic if it encounters any errors during the download process.
///
pub fn get_bsp_file(file_name: &str, update_file: bool, minutes: f64) -> Result<(), String> {
    // Take in the file name and check whether it exists and if the user wants to update the file (if it already exists)
    let file_path = format!("data/{}", file_name);

    // If file exists or update file is false, print to the terminal
    if !update_file && file_exists(&file_path) {
        println!("File {} already exists. Skipping download.", file_name);
        Ok(())
    } else {
        println!("Downloading {}", file_name);
        // Clone the filename and wrap it in an Arc for sharing between threads
        let file_name_clone = Arc::new(file_name.to_string());

        // Start a timer thread
        let timer_thread = thread::spawn({
            let file_name_clone = Arc::clone(&file_name_clone);

            move || {
                let start_time = Instant::now();
                // Set the time limit to 5 minutes for large files
                let time_limit = Duration::from_secs((minutes * 60.0) as u64);

                loop {
                    let elapsed = start_time.elapsed();
                    print!(
                        // Clear the rest of the line
                        "\rTime elapsed: {:.1} seconds                  ",
                        elapsed.as_secs_f64()
                    );
                    // Flush stdout to make the output visible immediately
                    std::io::stdout().flush().unwrap();

                    // Check if the time limit has been reached
                    if elapsed >= time_limit {
                        println!(
                            "\nTime limit of {:?} seconds ({:?} minutes) reached. Failed to download file. Exiting timer thread.",
                            time_limit,
                            time_limit / 60
                        );
                        // Exit the loop when the time limit is reached
                        break;
                    }

                    // Check if the file has been downloaded
                    if file_exists(&format!("data/{}", &file_name_clone)) == true {
                        println!(
                            "\nTime taken {:?}. File {} downloaded and saved successfully.",
                            elapsed, &file_name_clone,
                        );
                        // Exit the loop when the time limit is reached
                        break;
                    }

                    // Sleep for a short duration (e.g., 1 second)
                    thread::sleep(Duration::from_secs(1));
                }
            }
        });

        // Simulate another operation in the main thread (e.g., a function)
        match download_bsp_file(&file_name_clone, &file_path, minutes) {
            Ok(_) => {}
            Err(err) => {
                println!("");
                return Err(format!("Error downloading .bsp file: {}", err));
            }
        }
        thread::sleep(Duration::from_secs(2));

        // Wait for the timer thread to finish
        timer_thread.join().unwrap();

        Ok(())
    }
}

/// Downloads a BSP (Binary Space Partitioning) file if it does not exist or if an update is requested.
///
/// # Arguments
///
/// * `file` - The name of the BSP file to download.
/// * `file_path` - The local file path where the BSP file will be saved.
/// * `minutes` - The maximum number of minutes to allow for the download operation.
///
/// # Errors
///
/// This function returns a `Result<(), Box<dyn Error>>`:
///
/// - If the file already exists but `update_file` is `true`, and the update fails.
/// - If any other error occurs during the HTTP request, downloading, or file saving.
// ///
pub fn download_bsp_file(file: &str, file_path: &str, minutes: f64) -> Result<(), Box<dyn Error>> {
    // Locate the URL for obtaining .bsp files
    let url = format!("https://ssd.jpl.nasa.gov/ftp/eph/planets/bsp/{}", file);

    // Create a custom HTTP client with a timeout duration
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs((minutes * 60.0) as u64))
        .build()?;

    // Get the response
    let resp = client.get(&url).send()?;

    if !resp.status().is_success() {
        return Err(format!("Failed to download {}: {:?}", file, resp.status()).into());
    }

    let body = resp.bytes()?;

    let mut out = File::create(file_path)?;

    out.write_all(&body)?;

    Ok(())
}

/// Converts a snake_case or kebab-case string to a readable, space-separated string in Title Case.
///
/// This function takes an input string in snake_case or kebab-case and converts it to a readable
/// string in Title Case, where each word is capitalized and separated by spaces. This function is
/// used when handling segments in DAF i.e. it takes 'MERCURY_BARYCENTER' and returns 'Mercury Barycenter'
///
/// # Arguments
///
/// * `input` - The input string in snake_case or kebab-case format.
///
/// # Returns
///
/// A `String` containing the converted string in Title Case.
pub fn convert_to_readable(input: &str) -> String {
    input
        .replace("_", " ")
        .split_whitespace()
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                Some(first_char) => {
                    let capitalized = first_char.to_uppercase();
                    let rest = chars.as_str().to_lowercase();
                    format!("{}{}", capitalized, rest)
                }
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Checks whether a given file is binary or not based on its metadata.
///
/// This function takes a reference to a `File` and checks whether the file is binary
/// or not based on its metadata. It examines whether the file was opened with the binary
/// flag. If metadata cannot be retrieved, the function defaults to treating the file as
/// binary.
///
/// # Arguments
///
/// * `file`: A reference to a `File` that needs to be checked for binary content.
///
/// # Returns
///
/// A boolean value indicating whether the file is binary (`true`) or not (`false`).
pub fn is_binary(file: &File) -> bool {
    // Get the open options used for this file
    if let Ok(metadata) = file.metadata() {
        // Check if the file was opened with the binary flag
        metadata.is_file()
    } else {
        // Default to treating the file as binary if metadata cannot be retrieved
        true
    }
}
