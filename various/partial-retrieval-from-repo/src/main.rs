use std::fs::{remove_dir, remove_file, rename, File};
use std::io::prelude::*;
use std::path::Path;
use flate2::read::GzDecoder;
use tar::Archive;

fn download_and_extract(repo_url: &str, temp_tar_gz_path: &Path, parent_path: &str, project_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let extract_path = format!("{}/{}", parent_path, project_path);

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
        if path.to_string_lossy().contains(&extract_path) {
            entry.unpack_in(".").expect("Failed to unpack");
        }
    });

    // Step 3: Clean up
    rename(&extract_path, project_path)?;
    remove_file(temp_tar_gz_path)?;
    remove_dir(parent_path)?;

    Ok(())
}

const REPOSITORY: &str = "chainsight-showcase";
const BRANCH: &str = "main";

fn main() {
    let project_name = "sp500";

    let tar_gz_file = format!("{}.tar.gz", BRANCH);
    let repo_url = format!("https://github.com/horizonx-tech/{}/archive/refs/heads/{}", REPOSITORY, tar_gz_file);
    let temp_tar_gz_path = Path::new(&tar_gz_file);
    let parent_path = format!("{}-{}", REPOSITORY, BRANCH);

    if let Err(e) = download_and_extract(&repo_url, temp_tar_gz_path, &parent_path, &project_name) {
        eprintln!("Error occurred: {}", e);
    }
}
