use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use flate2::read::GzDecoder;
use tar::Archive;

fn download_and_extract(repo_url: &str, temp_tar_gz_path: &Path, extract_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Step 1: Download the .tar.gz archive
    let response = ureq::get(repo_url).call();
    if response.is_err() {
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Request failed")));
    }

    let mut file = File::create(temp_tar_gz_path)?;
    let mut reader = response.unwrap().into_reader();
    let mut content = Vec::new();
    reader.read_to_end(&mut content)?;
    file.write_all(&content)?;

    // Step 2: Decompress and extract the specified folder
    let tar_gz = File::open(temp_tar_gz_path)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);

    // Extract only the specified folder
    archive.entries()?.filter_map(|e| e.ok()).for_each(|mut entry| {
        let path = entry.path().ok().unwrap();
        if path.to_string_lossy().contains(extract_path) {
            entry.unpack_in(".").expect("Failed to unpack");
        }
    });

    Ok(())
}

fn main() {
    let repo_url = "https://github.com/horizonx-tech/chainsight-showcase/archive/refs/heads/main.tar.gz";
    let temp_tar_gz_path = Path::new("main.tar.gz");
    let extract_path = "chainsight-showcase-main/sp500";

    if let Err(e) = download_and_extract(repo_url, temp_tar_gz_path, extract_path) {
        eprintln!("Error occurred: {}", e);
    }
}
