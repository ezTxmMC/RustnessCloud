use anyhow::{anyhow, Context, Error};
use reqwest::header::CACHE_CONTROL;
use crate::downloader::url_downloader;
use crate::group::groupversion::{get_all_versions, get_url, software_and_version_exist, Manifest};

pub(crate) async fn download(software_type: String, software: String, version: String) -> Result<(), Error> {
    println!("{} - {}_{}", software, software, version);
    let manifest = get_manifest().await.context("Fehler beim Abrufen des Manifests")?;
    println!("{} - {:?}", software, get_all_versions(&manifest, &software_type, &software));
    if software_and_version_exist(&manifest, &software_type, &software, &version) {
        println!("{} is existing.", &software);
        let name = format!("{}-{}.jar", software.to_lowercase(), version.replace("_", "."));
        let url = get_url(&manifest, &software_type, &software, &version).unwrap();
        println!("{} - {}", name, url);
        url_downloader::download(&name, &url).await.map_err(|e| anyhow!("Download fehlgeschlagen: {}", e))?;
        return Ok(());
    }
    Err(anyhow!("{} version {} not found", software, version))
}

async fn get_manifest() -> Result<Manifest, Error> {
    let url = "https://raw.githubusercontent.com/ezTxmMC/rustnesscloud-manifest/refs/heads/master/versions.json";
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header(CACHE_CONTROL, "no-cache")
        .send()
        .await?
        .text()
        .await?;
    let manifest: Manifest = serde_json::from_str(&response)?;
    Ok(manifest)
}
