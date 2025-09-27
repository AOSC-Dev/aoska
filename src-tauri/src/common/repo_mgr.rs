use crate::common::config::LOCAL_REPO_PATH;
use crate::common::utils::run_cmd;
use ahash::{HashMap, HashMapExt};
use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use thiserror::Error;
use walkdir::WalkDir;

pub struct RepoManager {
    /// A HashMap to store deb files' path with a key of its name.
    packages: HashMap<String, PathBuf>,
}
#[derive(Error, Debug)]
pub enum RepoError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("package not found: {0}")]
    NotFound(String),

    // we could have better error handling.
    #[error("command error: {0}")]
    Other(#[from] anyhow::Error),
}

/// Local repo is for those packages which not included in repo.aosc.io
/// For example, packages which are not suitable for distributing.
impl RepoManager {
    pub fn new() -> Result<Self, RepoError> {
        let packages_map = Self::check_or_init_repo()?;
        Ok(Self {
            packages: packages_map,
        })
    }

    /// add a package to local repo.
    /// copy from temp folder.
    pub fn add_package(
        &mut self,
        package_name: String,
        package_path: PathBuf,
    ) -> Result<(), RepoError> {
        if Self::is_deb(&package_path) {
            return Err(
                std::io::Error::new(std::io::ErrorKind::InvalidInput, "not a deb file").into(),
            );
        }
        let pkgfile_name = format!("{package_name}.deb");
        let dst = Path::new(LOCAL_REPO_PATH).join(&pkgfile_name);
        // remove duplicated file first.
        // NOTE: we only keep the last one.
        fs::remove_file(&dst).ok();
        fs::copy(package_path, &dst)?;

        self.packages.insert(pkgfile_name.clone(), dst.clone());
        Self::generate_packages_index()?;
        Ok(())
    }

    pub fn remove_package(&mut self, package_name: &str) -> Result<(), RepoError> {
        let path = self
            .packages
            .remove(package_name)
            .ok_or_else(|| RepoError::NotFound(package_name.to_string()))?;
        fs::remove_file(path)?;
        Ok(())
    }

    /// check local repo exisitence
    /// if the repo does not exist, try to create it.
    fn check_or_init_repo() -> Result<HashMap<String, PathBuf>, RepoError> {
        let repo_path = Path::new(LOCAL_REPO_PATH);
        fs::create_dir_all(repo_path)?;
        let mut packages_map: HashMap<String, PathBuf> = HashMap::new();
        // search all deb files in the repo and build packages_map index.
        for entry in WalkDir::new(LOCAL_REPO_PATH)
            .follow_links(false)
            .into_iter()
            .filter_map(Result::ok)
        {
            if !entry.file_type().is_file() {
                continue;
            } // ignore non-file.
            let path = entry.into_path();
            if Self::is_deb(&path) {
                let filename = path
                    .file_name()
                    .and_then(|os| os.to_str())
                    .unwrap_or_default()
                    .to_string();
                let pkg_name = Self::extract_name(&filename).unwrap();
                packages_map.entry(pkg_name).insert_entry(path);
            }
        }
        Self::generate_packages_index()?;
        Ok(packages_map)
    }

    fn generate_packages_index() -> Result<(), RepoError> {
        let mut cmd = Command::new("dpkg-scanpackages");
        let repo_path = Path::new(LOCAL_REPO_PATH);
        cmd.arg(LOCAL_REPO_PATH).arg("/dev/null");
        let out = run_cmd(cmd)?;
        fs::write(repo_path.join("Packages"), out)?;
        Ok(())
    }

    fn is_deb(p: &Path) -> bool {
        p.extension()
            .and_then(|s| s.to_str())
            .is_some_and(|e| e.eq_ignore_ascii_case("deb"))
    }

    /// deb name definition: <package_name>.deb
    fn extract_name(filename: &str) -> Option<String> {
        let re = Regex::new(r"^([A-Za-z0-9\-\+\.])+(?:\.deb)?$").ok()?;
        re.captures(filename)
            .and_then(|cap| cap.get(1).map(|m| m.as_str().to_string()))
    }
}
