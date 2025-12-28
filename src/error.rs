use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
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
}
