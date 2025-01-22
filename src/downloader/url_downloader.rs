use std::{fs::File, io::Write};

pub(crate) async fn download(name: &str, url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?;

    let mut dest = File::create(name)?;
    let content = response.bytes().await?;
    dest.write_all(&content)?;

    println!("Downloaded {}", name);
    Ok(())
}