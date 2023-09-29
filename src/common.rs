use std::{fmt::Display, path::Path};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DataFormat {
    Json,
    JsonLines,
    #[cfg(feature = "csv")]
    Csv,
    #[cfg(feature = "yaml")]
    Yaml,
}

impl TryFrom<&str> for DataFormat {
    type Error = String;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        match value.trim().to_lowercase().as_str() {
            "json" => Ok(DataFormat::Json),
            "jsonl" | "jsl" => Ok(DataFormat::JsonLines),
            #[cfg(feature = "csv")]
            "csv" => Ok(DataFormat::Csv),
            #[cfg(feature = "yaml")]
            "yaml" | "yml" => Ok(DataFormat::Yaml),
            _ => Err(format!("Unknown data format: {}", value)),
        }
    }
}

impl TryFrom<&Path> for DataFormat {
    type Error = String;

    fn try_from(value: &Path) -> std::result::Result<Self, Self::Error> {
        let ext = value
            .extension()
            .ok_or_else(|| format!("No extension found for file: {}", value.display()))
            .and_then(|v| v.to_str().ok_or("Invalid extension".to_owned()))?;
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
        }
    }
}

pub(crate) type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Sync + Send>>;

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
    }
}
