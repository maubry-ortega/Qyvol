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
