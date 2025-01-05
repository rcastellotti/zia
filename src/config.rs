use serde::Deserialize;
use std::collections::HashMap;
use std::fmt::Display;
use std::fs;
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub bin1: PathBuf,
    pub bin2: PathBuf,
    pub commands: Vec<Command>,
}
#[derive(Deserialize, Debug)]
pub struct Command {
    pub name: String,
    pub env: Option<HashMap<String, String>>,
    pub args: Vec<String>,
}

#[derive(Debug)]
pub enum ZiaConfigError {
    LoadError(std::io::Error, PathBuf),
    DeserializationError(toml::de::Error),
    BinaryNotFoundError(String),
}

impl From<toml::de::Error> for ZiaConfigError {
    fn from(value: toml::de::Error) -> Self {
        ZiaConfigError::DeserializationError(value)
    }
}

impl Display for ZiaConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ZiaConfigError::LoadError(e, config_filename) => {
                write!(
                    f,
                    "Could not read config file {:?}: {}",
                    config_filename.display(),
                    e
                )
            }
            ZiaConfigError::DeserializationError(e) => {
                write!(f, "Could not parse config file: {}", e)
            }
            ZiaConfigError::BinaryNotFoundError(binary) => {
                write!(f, "Binary not found or not executable: {}", binary)
            }
        }
    }
}

impl Config {
    pub fn load_from_str(config_content: &str) -> Result<Config, ZiaConfigError> {
        let config: Config = toml::from_str(config_content)?;

        if !std::path::Path::new(&config.bin1).exists() {
            return Err(ZiaConfigError::BinaryNotFoundError(
                config.bin1.clone().display().to_string(),
            ));
        }
        if !std::path::Path::new(&config.bin2).exists() {
            return Err(ZiaConfigError::BinaryNotFoundError(
                config.bin2.clone().display().to_string(),
            ));
        }

        Ok(config)
    }

    pub fn load_from_file(config_filename: &str) -> Result<Config, ZiaConfigError> {
        log::info!("Opening config file: {}", config_filename);
        let config_file = fs::read_to_string(config_filename)
            .map_err(|err| ZiaConfigError::LoadError(err, config_filename.into()))?;
        Config::load_from_str(&config_file)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_load_from_str_valid() {
        let toml_content = r#"
            bin1 = "/usr/bin/valid_bin1"
            bin2 = "/usr/bin/valid_bin2"

            [[commands]]
            name = "list animals"
            env = { ZIA_ENV1 = "rob", ZIA_ENV2 = "f" }
            args = ["add", "Giorgio", "Scuttlebuff"]
        "#;

        // Mock binary existence
        let _mock_bin1 = NamedTempFile::new().unwrap();
        let _mock_bin2 = NamedTempFile::new().unwrap();

        let config_content = toml_content
            .replace("/usr/bin/valid_bin1", _mock_bin1.path().to_str().unwrap())
            .replace("/usr/bin/valid_bin2", _mock_bin2.path().to_str().unwrap());

        let result = Config::load_from_str(&config_content);
        assert!(result.is_ok());
    }

    #[test]
    fn test_load_from_str_invalid_binary() {
        let toml_content = r#"
            bin1 = "/nonexistent/bin1"
            bin2 = "/usr/bin/valid_bin2"

            [[commands]]
            name = "list animals"
            env = { ZIA_ENV1 = "rob", ZIA_ENV2 = "f" }
            args = ["add", "Giorgio", "Scuttlebuff"]
        "#;

        let _mock_bin2 = NamedTempFile::new().unwrap();
        let config_content =
            toml_content.replace("/usr/bin/valid_bin2", _mock_bin2.path().to_str().unwrap());

        let result = Config::load_from_str(&config_content);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(ZiaConfigError::BinaryNotFoundError(_))
        ));

        if let Err(ZiaConfigError::BinaryNotFoundError(bin)) = result {
            assert_eq!(bin, "/nonexistent/bin1");
        } else {
            panic!("Unexpected error type");
        }
    }

    #[test]
    fn test_load_from_str_missing_values() {
        let toml_content = r#"
            bin1 = "temp1"
            bin2 = "temp2"
        "#;

        let mut temp_file = NamedTempFile::new().unwrap();
        write!(temp_file, "{}", toml_content).unwrap();

        let result = Config::load_from_file(temp_file.path().to_str().unwrap());
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(ZiaConfigError::DeserializationError(_))
        ));
    }

    #[test]
    fn test_load_from_file_invalid_file() {
        let invalid_path = "/nonexistent/config/file.toml";
        let result = Config::load_from_file(invalid_path);
        assert!(result.is_err());
        assert!(matches!(result, Err(ZiaConfigError::LoadError(_, _))));
    }

    #[test]
    fn test_load_from_file_invalid_toml() {
        let invalid_toml_content = "invalid toml content";

        let mut temp_file = NamedTempFile::new().unwrap();
        write!(temp_file, "{}", invalid_toml_content).unwrap();

        let result = Config::load_from_file(temp_file.path().to_str().unwrap());
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(ZiaConfigError::DeserializationError(_))
        ));
    }
}
