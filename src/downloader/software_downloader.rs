/*fn downloadSpigot(supportedVersions: SupportedVersions) {
    let name = "spigot.jar";
    let url = "";
    let _ = downloader::download(name, url).await;
}*/

fn download_paper(supported_versions: SupportedVersions) {
    let name = "paper.jar";
    let url = "https://api.papermc.io/v2/projects/paper/versions/1.20.6/builds/78/downloads/paper-1.20.6-78.jar";
    let _ = downloader::download(name, url).await;
}

fn download_purpur(supported_versions: SupportedVersions) {
    let name = "purpur.jar";
    let url = "https://api.purpurmc.org/v2/purpur/1.20.6/2207/download";
    let _ = downloader::download(name, url).await;
}

fn download_folia(supported_versions: SupportedVersions) {
    let name = "folia.jar";
    let url = "https://api.papermc.io/v2/projects/folia/versions/1.20.6/builds/5/downloads/folia-1.20.6-5.jar";
    let _ = downloader::download(name, url).await;
}
