/*fn downloadSpigot(supportedVersions: SupportedVersions) {
    let name = "spigot.jar";
    let url = "";
    let _ = downloader::download(name, url).await;
}*/

fn downloadPaper(supportedVersions: SupportedVersions) {
    let name = "paper.jar";
    let url = "https://api.papermc.io/v2/projects/paper/versions/1.20.6/builds/78/downloads/paper-1.20.6-78.jar";
    let _ = downloader::download(name, url).await;
}

fn downloadPurpur(supportedVersions: SupportedVersions) {
    let name = "purpur.jar";
    let url = "https://api.purpurmc.org/v2/purpur/1.20.6/2207/download";
    let _ = downloader::download(name, url).await;
}

fn downloadPufferfish(supportedVersions: SupportedVersions) {
    let name = "pufferfish.jar";
    let url = "https://ci.pufferfish.host/job/Pufferfish-1.20/49/artifact/build/libs/pufferfish-paperclip-1.20.4-R0.1-SNAPSHOT-reobf.jar";
    let _ = downloader::download(name, url).await;
}