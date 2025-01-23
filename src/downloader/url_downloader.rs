use std::{fs, fs::File, io::Write, path::Path};
use anyhow::Result;

pub(crate) async fn download(name: &str, url: &str) -> Result<()> {
    println!("Downloading {}", name);

    let storage_dir = Path::new("storage/versions");
    if !storage_dir.exists() {
        fs::create_dir_all(storage_dir)?;
        println!("Created directory: {:?}", storage_dir);
    }

    let file_path = storage_dir.join(name);

    let response = reqwest::get(url).await?;
    let content = response.bytes().await?;

    let mut dest = File::create(&file_path)?;
    dest.write_all(&content)?;

    println!("Downloaded {} to {:?}", name, file_path);
    Ok(())
}
