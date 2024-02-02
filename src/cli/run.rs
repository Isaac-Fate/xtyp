use std::path::{ PathBuf, Path };
use clap::Parser;
use crate::{ TypstPackageConfig, LOCAL_TYPST_PACKAGE_DIR };
use super::{ App, Commands };

impl App {
    /// Run the CLI app.
    pub fn run() {
        // Parse the arguments
        let args = Self::parse();

        match &args.command {
            // Install a local Typst package
            Some(Commands::Install { package_dir }) => {
                // Load the package configuration
                let config = match load_typst_package_config(package_dir) {
                    Ok(config) => config,
                    Err(err) => {
                        eprintln!("Failed to load the package configuration: {}", err);
                        std::process::exit(1);
                    }
                };

                // Get the target directory
                let target_dir = LOCAL_TYPST_PACKAGE_DIR.join(&config.package.name).join(
                    &config.package.version
                );

                // Copy the package directory to the target directory
                match copy_dir(package_dir, &target_dir) {
                    Ok(_) => {
                        println!("Package installed successfully to {:?}", &target_dir);
                    }
                    Err(err) => {
                        eprintln!("Failed to copy the package directory: {}", err);
                        std::process::exit(1);
                    }
                }
            }
            _ => {}
        }
    }
}

fn load_typst_package_config(
    package_dir: &PathBuf
) -> Result<TypstPackageConfig, Box<dyn std::error::Error>> {
    // Package configuration file
    let config_filepath = package_dir.join("typst.toml");

    // Read the file content
    let config_file_content = std::fs::read_to_string(config_filepath)?;

    // Parse to struct
    let config: TypstPackageConfig = toml::from_str(&config_file_content)?;

    Ok(config)
}

fn copy_dir<P: AsRef<Path>>(from: P, to: P) -> std::io::Result<()> {
    match std::fs::remove_dir_all(&to) {
        Ok(_) => {}
        Err(error) => {
            // Ignore the error if the directory does not exist
            assert!(matches!(error.kind(), std::io::ErrorKind::NotFound));
        }
    }

    // Copy the directory
    let result = match copy_dir::copy_dir(from, &to) {
        Ok(_) => { Ok(()) }
        Err(error) => {
            eprintln!("Failed to copy the directory: {}", error);
            Err(error)
        }
    };

    // Remove .git dir
    let git_dir = to.as_ref().join(".git");
    match std::fs::remove_dir_all(&git_dir) {
        Ok(_) => { result }
        Err(error) => {
            // Ignore the error if the directory does not exist
            assert!(matches!(error.kind(), std::io::ErrorKind::NotFound));
            Err(error)
        }
    }
}
