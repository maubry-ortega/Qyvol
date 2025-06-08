// # VolleyDevByMaubry [3/∞] "Los manifiestos son oráculos, no archivos."
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Manifest {
    pub name: String,
    pub entrypoint: String,
    pub runtime: String,
}

// common/src/lib.rs (agrega abajo del struct)
use std::fs;
use std::path::{Path, PathBuf};

impl Manifest {
    pub fn from_file(path: &Path) -> Result<(Manifest, PathBuf), Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let manifest: Manifest = serde_yaml::from_str(&content)?;
        let base_dir = path.parent().unwrap_or(Path::new(".")).to_path_buf();
        Ok((manifest, base_dir))
    }
}

