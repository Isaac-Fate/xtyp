use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TypstPackageConfig {
    pub package: Package,
}

#[derive(Debug, Deserialize)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub entrypoint: String,
    pub description: String,
}
