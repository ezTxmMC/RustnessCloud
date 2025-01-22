use crate::downloader::url_downloader;

async fn download_bungeecord() {
    let name = "bungeecord.jar";
    let url = "https://ci.md-5.net/job/BungeeCord/lastSuccessfulBuild/artifact/bootstrap/target/BungeeCord.jar";
    let _ = url_downloader::download(name, url).await;
}

async fn download_velocity() {
    let name = "velocity.jar";
    let url = "https://api.papermc.io/v2/projects/velocity/versions/3.3.0-SNAPSHOT/builds/390/downloads/velocity-3.3.0-SNAPSHOT-390.jar";
    let _ = url_downloader::download(name, url).await;
}