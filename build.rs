use std::{fs::File, path::Path};

fn main() {
    // Download and unzip `enwik9` data source
    let url = "https://mattmahoney.net/dc/enwik9.zip";
    let path = Path::new("target/enwik9.zip");
    if !path.exists() {
        let _ = std::fs::create_dir_all(path.parent().unwrap());

        let mut resp = reqwest::blocking::get(url).unwrap();
        let mut out = File::create(path).unwrap();
        std::io::copy(&mut resp, &mut out).unwrap();

        // Unzip data source
        let status = std::process::Command::new("unzip")
            .arg("target/enwik9.zip")
            .arg("-d")
            .arg("target")
            .status()
            .unwrap();
        assert!(status.success());
    }
}
