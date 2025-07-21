use std::fs;
use std::path::PathBuf;
use anyhow::Result;
use chrono::Utc;
use serde::Deserialize;
use aoska::common::packages::{Category, PackageBrief, PackageDetail, PackageFlags, PackageInfo};
use aoska::common::index::{Index, CategoryIndex, RecommendIndex};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "aoska_maintainer")]
#[command(about = "A tool to generate JSON files from TOML configuration")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate index JSON file from TOML configuration
    GenerateIndex {
        /// Input TOML file path
        #[arg(short, long)]
        input: PathBuf,
        /// Output JSON file path
        #[arg(short, long)]
        output: PathBuf,
    },
    /// Generate recommend index JSON file from TOML configuration
    GenerateRecommend {
        /// Input TOML file path
        #[arg(short, long)]
        input: PathBuf,
        /// Output JSON file path
        #[arg(short, long)]
        output: PathBuf,
    },
    /// Generate package detail JSON file from TOML configuration
    GeneratePackage {
        /// Input TOML file path
        #[arg(short, long)]
        input: PathBuf,
        /// Output JSON file path
        #[arg(short, long)]
        output: PathBuf,
    },
}

#[derive(Debug, Deserialize)]
struct TomlIndex {
    version: u8,
    categories: Vec<TomlCategoryIndex>,
}

#[derive(Debug, Deserialize)]
struct TomlCategoryIndex {
    category: Category,
    packages: Vec<TomlPackageBrief>,
}

#[derive(Debug, Deserialize)]
struct TomlPackageBrief {
    name: String,
    intro: String,
    icon: PathBuf,
}

#[derive(Debug, Deserialize)]
struct TomlRecommendIndex {
    packages: Vec<TomlPackageBrief>,
}

#[derive(Debug, Deserialize)]
struct TomlPackageDetail {
    name: String,
    icon: PathBuf,
    title: String,
    intro: String,
    category: Category,
    screenshot: Vec<PathBuf>,
    package_flags: TomlPackageFlags,
    package_info: TomlPackageInfo,
}

#[derive(Debug, Deserialize)]
struct TomlPackageFlags {
    unoffical: bool,
    verified: bool,
    non_native: bool,
    windows_app: bool,
    telemetry: bool,
    service_limited: bool,
}

#[derive(Debug, Deserialize)]
struct TomlPackageInfo {
    publisher: String,
    source: String,
    version: String,
    inner_version: i32,
    update_date: String,
    install_size: i64,
    homepage: String,
}

impl From<TomlPackageBrief> for PackageBrief {
    fn from(toml_brief: TomlPackageBrief) -> Self {
        PackageBrief {
            name: toml_brief.name,
            intro: toml_brief.intro,
            icon: toml_brief.icon,
        }
    }
}

impl From<TomlCategoryIndex> for CategoryIndex {
    fn from(toml_category: TomlCategoryIndex) -> Self {
        CategoryIndex {
            category: toml_category.category,
            packages: toml_category.packages.into_iter().map(|p| p.into()).collect(),
        }
    }
}

impl From<TomlPackageFlags> for PackageFlags {
    fn from(toml_flags: TomlPackageFlags) -> Self {
        PackageFlags {
            unoffical: toml_flags.unoffical,
            verified: toml_flags.verified,
            non_native: toml_flags.non_native,
            windows_app: toml_flags.windows_app,
            telemetry: toml_flags.telemetry,
            service_limited: toml_flags.service_limited,
        }
    }
}

impl From<TomlPackageInfo> for PackageInfo {
    fn from(toml_info: TomlPackageInfo) -> Self {
        PackageInfo {
            publisher: toml_info.publisher,
            source: toml_info.source,
            version: toml_info.version,
            inner_version: toml_info.inner_version,
            update_date: toml_info.update_date,
            install_size: toml_info.install_size,
            homepage: toml_info.homepage,
        }
    }
}

impl From<TomlPackageDetail> for PackageDetail {
    fn from(toml_detail: TomlPackageDetail) -> Self {
        PackageDetail {
            name: toml_detail.name,
            icon: toml_detail.icon,
            title: toml_detail.title,
            intro: toml_detail.intro,
            category: toml_detail.category,
            screenshot: toml_detail.screenshot,
            package_flags: toml_detail.package_flags.into(),
            package_info: toml_detail.package_info.into(),
        }
    }
}

fn generate_index(input: PathBuf, output: PathBuf) -> Result<()> {
    let toml_content = fs::read_to_string(&input)?;
    let toml_index: TomlIndex = toml::from_str(&toml_content)?;
    
    let index = Index {
        version: toml_index.version,
        generated_at: Utc::now(),
        packages: toml_index.categories.into_iter().map(|c| c.into()).collect(),
    };
    
    let json_content = serde_json::to_string_pretty(&index)?;
    fs::write(&output, json_content)?;
    
    println!("Generated index JSON file: {:?}", output);
    Ok(())
}

fn generate_recommend(input: PathBuf, output: PathBuf) -> Result<()> {
    let toml_content = fs::read_to_string(&input)?;
    let toml_recommend: TomlRecommendIndex = toml::from_str(&toml_content)?;
    
    let recommend_index = RecommendIndex {
        date: Utc::now(),
        packages: toml_recommend.packages.into_iter().map(|p| p.into()).collect(),
    };
    
    let json_content = serde_json::to_string_pretty(&recommend_index)?;
    fs::write(&output, json_content)?;
    
    println!("Generated recommend index JSON file: {:?}", output);
    Ok(())
}

fn generate_package(input: PathBuf, output: PathBuf) -> Result<()> {
    let toml_content = fs::read_to_string(&input)?;
    let toml_package: TomlPackageDetail = toml::from_str(&toml_content)?;
    
    let package_detail: PackageDetail = toml_package.into();
    
    let json_content = serde_json::to_string_pretty(&package_detail)?;
    fs::write(&output, json_content)?;
    
    println!("Generated package detail JSON file: {:?}", output);
    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::GenerateIndex { input, output } => {
            generate_index(input, output)?;
        }
        Commands::GenerateRecommend { input, output } => {
            generate_recommend(input, output)?;
        }
        Commands::GeneratePackage { input, output } => {
            generate_package(input, output)?;
        }
    }
    
    Ok(())
}
