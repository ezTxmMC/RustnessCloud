use std::{fs::File, io::Write};

pub(crate) async fn download(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?;

    let mut dest = File::create("example.txt")?;
    let content = response.bytes().await?;
    dest.write_all(&content)?;

    println!("Downloaded to: example.txt");
    Ok(())
}