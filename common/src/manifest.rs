// # VolleyDevByMaubry [3/∞] "Los manifiestos son oráculos, no archivos."
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize, PartialEq)]
pub struct Permissions {
    pub fs: Option<String>,
    pub net: bool,
    pub exec: bool,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Manifest {
    pub name: String,
    pub entrypoint: String,
    pub runtime: String,
    pub permissions: Option<Permissions>,
}

impl Manifest {
    pub fn from_file(path: &Path) -> Result<(Manifest, PathBuf), Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let manifest: Manifest = serde_yaml::from_str(&content)?;
        let base_dir = path.parent().unwrap_or(Path::new(".")).to_path_buf();
        Ok((manifest, base_dir))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manifest_parse_basic() {
        let yaml = r#"
name: hello-qyvol
entrypoint: hello.wasm
runtime: wasi
"#;
        let manifest: Manifest = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(manifest.name, "hello-qyvol");
        assert_eq!(manifest.entrypoint, "hello.wasm");
        assert_eq!(manifest.runtime, "wasi");
        assert_eq!(manifest.permissions, None);
    }

    #[test]
    fn test_manifest_parse_with_permissions() {
        let yaml = r#"
name: hello-qyvol
entrypoint: hello.wasm
runtime: wasi
permissions:
  fs: none
  net: false
  exec: false
"#;
        let manifest: Manifest = serde_yaml::from_str(yaml).unwrap();
        let expected_permissions = Permissions {
            fs: Some("none".to_string()),
            net: false,
            exec: false,
        };
        assert_eq!(manifest.permissions, Some(expected_permissions));
    }
}