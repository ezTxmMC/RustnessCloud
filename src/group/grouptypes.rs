pub(crate) enum ServiceType {
    PROXY,
    LOBBY,
    SERVER
}

pub(crate) enum ProxySoftwareType {
    BUNGEECORD,
    VELOCITY
}

pub(crate) enum ServerSoftwareType {
    PAPER,
    PURPUR,
    FOLIA,
    MINESTOM
}

pub(crate) enum SupportedVersions {
    V1_19_4,
    V1_20_1,
    V1_20_4,
    V1_20_6,
    V1_21_1,
    V1_21_3,
    V1_21_4,
}

pub(crate) enum StoreType {
    PERMANENTLY,
    TEMPORARY
}