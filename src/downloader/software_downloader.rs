use anyhow::{anyhow, Error};
use serde::Deserialize;
use crate::downloader::url_downloader;

// Lädt die neueste Version von Paper herunter
pub(crate) async fn download_paper(version: String) -> Result<(), Error> {
    let name = format!("paper-{}.jar", version.replace("_", "."));
    let url = "https://api.papermc.io/v2/projects/paper/versions/1.21.4/builds/121/downloads/paper-1.21.4-121.jar";
    url_downloader::download(&name, &url).await.map_err(|e| anyhow!("Download fehlgeschlagen: {}", e))?;
    Ok(())
}

// Lädt die neueste Version von Purpur herunter
pub(crate) async fn download_purpur(version: String) -> Result<(), Error> {
    let name = format!("purpur-{}.jar", version);
    let url = "https://api.purpurmc.org/v2/purpur/1.20.6/2207/download";

    url_downloader::download(&name, url).await.map_err(|e| anyhow!("Download fehlgeschlagen: {}", e))?;
    Ok(())
}

// Lädt die neueste Version von Folia herunter
pub(crate) async fn download_folia(version: String) -> Result<(), Error> {
    let name = format!("folia-{}.jar", version);
    let url = "https://api.papermc.io/v2/projects/folia/versions/1.20.6/builds/5/downloads/folia-1.20.6-5.jar";

    url_downloader::download(&name, url).await.map_err(|e| anyhow!("Download fehlgeschlagen: {}", e))?;
    Ok(())
}

