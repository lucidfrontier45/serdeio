use std::{fmt::Display, path::Path};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FileFormat {
    Json,
    JsonLines,
    #[cfg(feature = "csv")]
    Csv,
    #[cfg(feature = "yaml")]
    Yaml,
}

impl TryFrom<&str> for FileFormat {
    type Error = String;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        match value.trim().to_lowercase().as_str() {
            "json" => Ok(FileFormat::Json),
            "jsonl" | "jsl" => Ok(FileFormat::JsonLines),
            #[cfg(feature = "yaml")]
            "csv" => Ok(FileFormat::Csv),
            #[cfg(feature = "yaml")]
            "yaml" | "yml" => Ok(FileFormat::Yaml),
            _ => Err(format!("Unknown file format: {}", value)),
        }
    }
}

impl TryFrom<&Path> for FileFormat {
    type Error = String;

    fn try_from(value: &Path) -> std::result::Result<Self, Self::Error> {
        let ext = value
            .extension()
            .ok_or_else(|| format!("No extension found for file: {}", value.display()))
            .and_then(|v| v.to_str().ok_or("Invalid extension".to_owned()))?;
        Self::try_from(ext)
    }
}

impl Display for FileFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileFormat::Json => write!(f, "json"),
            FileFormat::JsonLines => write!(f, "jsonl"),
            #[cfg(feature = "csv")]
            FileFormat::Csv => write!(f, "csv"),
            #[cfg(feature = "yaml")]
            FileFormat::Yaml => write!(f, "yaml"),
        }
    }
}

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[cfg(test)]
mod test {
    #[test]
    fn test_file_format() {
        use super::FileFormat;
        use std::convert::TryFrom;
        assert_eq!(FileFormat::try_from("json").unwrap(), FileFormat::Json);
        assert_eq!(
            FileFormat::try_from("jsonl").unwrap(),
            FileFormat::JsonLines
        );
        assert_eq!(FileFormat::try_from("JSON").unwrap(), FileFormat::Json);
        assert_eq!(
            FileFormat::try_from("JSONL").unwrap(),
            FileFormat::JsonLines
        );
    }
}
