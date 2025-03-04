use bitcoin_hashes::{sha256, Hash};
use std::fs::File;
use std::io::{BufRead, BufReader, Cursor, Read};
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::str::FromStr;

include!("src/versions.rs");

const GITHUB_URL: &str = "https://github.com/RCasatta/electrsd/releases/download/electrs_releases";

fn get_expected_sha256(filename: &str) -> Result<sha256::Hash, ()> {
    let file = File::open("sha256").map_err(|_| ())?;
    for line in BufReader::new(file).lines().flatten() {
        let tokens: Vec<_> = line.split("  ").collect();
        if tokens.len() == 2 && filename == tokens[1] {
            return sha256::Hash::from_str(tokens[0]).map_err(|_| ());
        }
    }
    Err(())
}

fn main() {
    if !HAS_FEATURE {
        return;
    }
    let download_filename_without_extension = electrs_name();
    let download_filename = format!("{}.zip", download_filename_without_extension);
    dbg!(&download_filename);
    let expected_hash = get_expected_sha256(&download_filename).unwrap();
    let electrs_exe_home = format!("{}/electrs", std::env::var("CARGO_HOME").unwrap());
    let destination_filename: PathBuf = format!(
        "{}/{}/electrs",
        &electrs_exe_home, download_filename_without_extension
    )
    .into();
    dbg!(&destination_filename);

    if !destination_filename.exists() {
        println!(
            "filename:{} version:{} hash:{}",
            download_filename, VERSION, expected_hash
        );

        let url = format!("{}/{}", GITHUB_URL, download_filename);
        let mut downloaded_bytes = Vec::new();

        let _size = ureq::get(&url)
            .call()
            .unwrap()
            .into_reader()
            .read_to_end(&mut downloaded_bytes)
            .unwrap();

        let downloaded_hash = sha256::Hash::hash(&downloaded_bytes);
        assert_eq!(expected_hash, downloaded_hash);
        let cursor = Cursor::new(downloaded_bytes);

        let mut archive = zip::ZipArchive::new(cursor).unwrap();
        let mut file = archive.by_index(0).unwrap();
        std::fs::create_dir_all(destination_filename.parent().unwrap()).unwrap();
        let mut outfile = std::fs::File::create(&destination_filename).unwrap();

        std::io::copy(&mut file, &mut outfile).unwrap();
        std::fs::set_permissions(
            &destination_filename,
            std::fs::Permissions::from_mode(0o755),
        )
        .unwrap();
    }
}
