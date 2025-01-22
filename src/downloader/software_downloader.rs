use crate::downloader::url_downloader;
use crate::group::grouptypes::SupportedVersions;

async fn download_paper(supported_versions: SupportedVersions) {
    let name = "paper.jar";
    let url = "https://api.papermc.io/v2/projects/paper/versions/1.20.6/builds/78/downloads/paper-1.20.6-78.jar";
    let _ = url_downloader::download(name, url).await;
}

async fn download_purpur(supported_versions: SupportedVersions) {
    let name = "purpur.jar";
    let url = "https://api.purpurmc.org/v2/purpur/1.20.6/2207/download";
    let _ = url_downloader::download(name, url).await;
}

async fn download_folia(supported_versions: SupportedVersions) {
    let name = "folia.jar";
    let url = "https://api.papermc.io/v2/projects/folia/versions/1.20.6/builds/5/downloads/folia-1.20.6-5.jar";
    let _ = url_downloader::download(name, url).await;
}
