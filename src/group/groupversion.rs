use serde::Deserialize;
use reqwest;

#[derive(Deserialize)]
pub(crate) struct Manifest {
    PROXY: Option<Category>,
    SERVER: Option<Category>,
}

#[derive(Deserialize)]
pub(crate) struct Category {
    VELOCITY: Option<Software>,
    BUNGEECORD: Option<Software>,
    WATERFALL: Option<Software>,
    PAPER: Option<Software>,
    PUFFERFISH: Option<Software>,
    PURPUR: Option<Software>,
    FOLIA: Option<Software>,
    VANILLA: Option<Software>,
}

#[derive(Deserialize)]
pub(crate) struct Software {
    #[serde(flatten)]
    versions: std::collections::HashMap<String, Version>,
}

#[derive(Deserialize)]
pub(crate) struct Version {
    url: String,
}

pub(crate) fn software_and_version_exist(manifest: &Manifest, category: &str, software: &str, version: &str) -> bool {
    let category = match category {
        "PROXY" => &manifest.PROXY,
        "SERVER" => &manifest.SERVER,
        _ => return false,
    };

    let category = match category {
        Some(cat) => cat,
        None => return false,
    };

    let software = match software {
        "VELOCITY" => &category.VELOCITY,
        "BUNGEECORD" => &category.BUNGEECORD,
        "WATERFALL" => &category.WATERFALL,
        "PAPER" => &category.PAPER,
        "PUFFERFISH" => &category.PUFFERFISH,
        "PURPUR" => &category.PURPUR,
        "FOLIA" => &category.FOLIA,
        "VANILLA" => &category.VANILLA,
        _ => return false,
    };

    let software = match software {
        Some(soft) => soft,
        None => return false,
    };

    software.versions.contains_key(version)
}

pub(crate) fn get_all_softwares(manifest: &Manifest, category: &str) -> Option<Vec<String>> {
    let category = match category {
        "PROXY" => manifest.PROXY.as_ref(),
        "SERVER" => manifest.SERVER.as_ref(),
        _ => return None,
    }?;

    let mut softwares = Vec::new();
    if category.VELOCITY.is_some() { softwares.push("VELOCITY".to_string()); }
    if category.BUNGEECORD.is_some() { softwares.push("BUNGEECORD".to_string()); }
    if category.WATERFALL.is_some() { softwares.push("WATERFALL".to_string()); }
    if category.PAPER.is_some() { softwares.push("PAPER".to_string()); }
    if category.PUFFERFISH.is_some() { softwares.push("PUFFERFISH".to_string()); }
    if category.PURPUR.is_some() { softwares.push("PURPUR".to_string()); }
    if category.FOLIA.is_some() { softwares.push("FOLIA".to_string()); }
    if category.VANILLA.is_some() { softwares.push("VANILLA".to_string()); }

    Some(softwares)
}

pub(crate) fn get_all_versions(manifest: &Manifest, category: &str, software: &str) -> Option<Vec<String>> {
    let category = match category {
        "PROXY" => manifest.PROXY.as_ref(),
        "SERVER" => manifest.SERVER.as_ref(),
        _ => return None,
    }?;

    let software = match software {
        "VELOCITY" => category.VELOCITY.as_ref(),
        "BUNGEECORD" => category.BUNGEECORD.as_ref(),
        "WATERFALL" => category.WATERFALL.as_ref(),
        "PAPER" => category.PAPER.as_ref(),
        "PUFFERFISH" => category.PUFFERFISH.as_ref(),
        "PURPUR" => category.PURPUR.as_ref(),
        "FOLIA" => category.FOLIA.as_ref(),
        "VANILLA" => category.VANILLA.as_ref(),
        _ => return None,
    }?;

    Some(software.versions.keys().cloned().collect())
}

pub(crate) fn get_url(manifest: &Manifest, category: &str, software: &str, version: &str) -> Option<String> {
    let category = match category {
        "PROXY" => manifest.PROXY.as_ref(),
        "SERVER" => manifest.SERVER.as_ref(),
        _ => return None,
    }?;

    let software = match software {
        "VELOCITY" => category.VELOCITY.as_ref(),
        "BUNGEECORD" => category.BUNGEECORD.as_ref(),
        "WATERFALL" => category.WATERFALL.as_ref(),
        "PAPER" => category.PAPER.as_ref(),
        "PUFFERFISH" => category.PUFFERFISH.as_ref(),
        "PURPUR" => category.PURPUR.as_ref(),
        "FOLIA" => category.FOLIA.as_ref(),
        "VANILLA" => category.VANILLA.as_ref(),
        _ => return None,
    }?;

    software.versions.get(version).map(|v| v.url.clone())
}
