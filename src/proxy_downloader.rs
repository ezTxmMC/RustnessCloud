fn downloadBungeecord() {
    let name = "bungeecord.jar";
    let url = "";
    let _ = downloader::download(name, url).await;
}

fn downloadVelocity() {
    let name = "velocity.jar";
    let url = "https://api.papermc.io/v2/projects/velocity/versions/3.3.0-SNAPSHOT/builds/390/downloads/velocity-3.3.0-SNAPSHOT-390.jar";
    let _ = downloader::download(name, url).await;
}