use std::{fmt::Display, path::Path};

use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DataFormat {
    Json,
    JsonLines,
    #[cfg(feature = "csv")]
    Csv,
    #[cfg(feature = "yaml")]
    Yaml,
    #[cfg(feature = "messagepack")]
    MessagePack,
}

#[derive(Error, Debug)]
pub enum DataFormatError {
    #[error("Unknown data format: {0}")]
    Unknown(String),
    #[error("No extension found for file: {}", .0.display())]
    NoExtension(std::path::PathBuf),
    #[error("Invalid extension")]
    InvalidExtension,
}

impl TryFrom<&str> for DataFormat {
    type Error = DataFormatError;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        match value.trim().to_lowercase().as_str() {
            "json" => Ok(DataFormat::Json),
            "jsonl" | "jsl" => Ok(DataFormat::JsonLines),
            #[cfg(feature = "csv")]
            "csv" => Ok(DataFormat::Csv),
            #[cfg(feature = "yaml")]
            "yaml" | "yml" => Ok(DataFormat::Yaml),
            #[cfg(feature = "messagepack")]
            "msgpack" | "mpack" | "mpk" => Ok(DataFormat::MessagePack),
            _ => Err(DataFormatError::Unknown(value.to_string())),
        }
    }
}

impl TryFrom<&Path> for DataFormat {
    type Error = DataFormatError;

    fn try_from(value: &Path) -> std::result::Result<Self, Self::Error> {
        let ext = value
            .extension()
            .ok_or_else(|| DataFormatError::NoExtension(value.to_path_buf()))
            .and_then(|v| v.to_str().ok_or(DataFormatError::InvalidExtension))?;
        Self::try_from(ext)
    }
}

impl Display for DataFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataFormat::Json => write!(f, "json"),
            DataFormat::JsonLines => write!(f, "jsonl"),
            #[cfg(feature = "csv")]
            DataFormat::Csv => write!(f, "csv"),
            #[cfg(feature = "yaml")]
            DataFormat::Yaml => write!(f, "yaml"),
            #[cfg(feature = "messagepack")]
            DataFormat::MessagePack => write!(f, "messagepack"),
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_data_format() {
        use super::DataFormat;
        use std::convert::TryFrom;
        assert_eq!(DataFormat::try_from("json").unwrap(), DataFormat::Json);
        assert_eq!(
            DataFormat::try_from("jsonl").unwrap(),
            DataFormat::JsonLines
        );
        assert_eq!(DataFormat::try_from("JSON").unwrap(), DataFormat::Json);
        assert_eq!(
            DataFormat::try_from("JSONL").unwrap(),
            DataFormat::JsonLines
        );
        #[cfg(feature = "csv")]
        assert_eq!(DataFormat::try_from("csv").unwrap(), DataFormat::Csv);
        #[cfg(feature = "yaml")]
        assert_eq!(DataFormat::try_from("yaml").unwrap(), DataFormat::Yaml);
        #[cfg(feature = "messagepack")]
        {
            assert_eq!(
                DataFormat::try_from("msgpack").unwrap(),
                DataFormat::MessagePack
            );
            assert_eq!(
                DataFormat::try_from("mpack").unwrap(),
                DataFormat::MessagePack
            );
            assert_eq!(
                DataFormat::try_from("mpk").unwrap(),
                DataFormat::MessagePack
            );
        }
    }
}
