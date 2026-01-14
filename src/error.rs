use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Auto data format is not supported for this operation")]
    AutoNotSupported,
    #[error("Data format error: {0}")]
    DataFormat(#[from] crate::types::DataFormatError),
    #[error("Unsupported file format: {0}")]
    UnsupportedFormat(crate::types::DataFormat),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[cfg(feature = "csv")]
    #[error("CSV error: {0}")]
    Csv(#[from] csv::Error),
    #[cfg(feature = "yaml")]
    #[error("YAML error: {0}")]
    Yaml(#[from] serde_yaml::Error),
    #[cfg(feature = "messagepack")]
    #[error("MessagePack decode error: {0}")]
    MessagePackDecode(#[from] rmp_serde::decode::Error),
    #[cfg(feature = "messagepack")]
    #[error("MessagePack encode error: {0}")]
    MessagePackEncode(#[from] rmp_serde::encode::Error),
    #[cfg(feature = "toml")]
    #[error("TOML deserialization error: {0}")]
    TomlDeserialize(#[from] toml::de::Error),
    #[cfg(feature = "toml")]
    #[error("TOML serialization error: {0}")]
    TomlSerialize(#[from] toml::ser::Error),
}
