pub mod arg;

use arg::Args;

use clap::Parser;

use std::fs;
use std::path::Path;
use std::process::exit;
use std::time::Duration;

const DATA_IMG_DIR: &str = "images";
const DATA_INPUT_FILE: &str = "input.txt";
const DATA_OUTPUT_FILE: &str = "output.txt";

const FLAG_CDN_URL: &str = "https://flagcdn.com";
const FLAG_CDN_SIZE: &str = "64x48";
const FLAG_CDN_EXT: &str = "png";

fn main() {
    let args = Args::parse();

    // First let's format the base URL for flags.
    let base_url = format!("{}/{}/{{{{short_code}}}}.{}", FLAG_CDN_URL, FLAG_CDN_SIZE, FLAG_CDN_EXT);

    // Before we read input, let's create the output file.
    let data_path = Path::new(&args.data);

    if !data_path.is_dir() {
        eprintln!("Provided data path is not a directory: {}", args.data);
        
        exit(1);
    }

    // Retrieve the parent directory of output file and try to create it if it doesn't exist.
    // Unsure if append mode will create parent directory if it doesn't exist, so let's do it manually.
    if !data_path.exists() {
        fs::create_dir_all(data_path).expect("Failed to create data directory.");

        println!("Created data directory: {}...", args.data);
    }

    // We need to scan the input file for all country codes.
    let input = fs::read_to_string(data_path.join(DATA_INPUT_FILE)).expect("Failed to read input file.");

    // Next, we must read the input file line by line.
    let lines = input.lines().map(|line| line.trim().to_lowercase()).filter(|line| !line.is_empty());

    let mut new_content = String::new();

    for line in lines {
        // Format the URL and replace code.
        let url = base_url.replace("{{short_code}}", line.as_str());

        let mut skipped = false;
        
        // Build client with custom timeout from arguments.
        let cl = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(args.timeout as u64))
            .build()
            .expect("Failed to build client.");

        // Send request to download flag.
        let res = match cl.get(&url).send() {
            Ok(res) => res,
            Err(err) => {
                eprintln!("[WARNING] Failed to download flag for short code: {}. Error: {}", line, err);

                continue;
            }
        };

        if res.status().is_success() {
            // Save file to images directory if it doesn't exist.
            let img_path = data_path.join(DATA_IMG_DIR).join(format!("{}.{}", line, FLAG_CDN_EXT));

            if !img_path.exists() {
                fs::create_dir_all(img_path.parent().unwrap()).expect("Failed to create images directory.");

                let img_data = res.bytes().expect("Failed to read image data.");

                fs::write(&img_path, img_data).expect("Failed to save image.");
            } else {
                skipped = true;
            }

            let output_line = args.fmt.replace("{{short_code}}", line.as_str());

            new_content.push_str(&output_line);
            new_content.push('\n');
        } else {
            eprintln!("[WARNING] Failed to download flag for short code: {}. Status: {} (URL => {})", line, res.status(), url);
            
            continue;
        }

        if !skipped {
            println!("Successfully downloaded flag for short code: {}...", line);
        } else {
            println!("Flag for short code: {} already exists, skipping download...", line);
        }
    }

    // Write the new content to the output file.
    fs::write(data_path.join(DATA_OUTPUT_FILE), new_content).expect("Failed to write to output file.");
}
