mod config;

pub use config::TypstPackageConfig;

use lazy_static::lazy_static;
use std::path::PathBuf;
lazy_static! {
    pub static ref LOCAL_TYPST_PACKAGE_DIR: PathBuf = {
        let path = if cfg!(target_os = "macos") {
            let home_dir = PathBuf::from(std::env::var("HOME").unwrap());
            home_dir.join("Library/Application Support/typst/packages/local")
        } else if cfg!(target_os = "linux") {
            let home_dir = PathBuf::from(std::env::var("HOME").unwrap());
            home_dir.join(".local/share/typst/packages/local")
        } else if cfg!(target_os = "windows") {
            let app_data_dir = PathBuf::from("%APPDATA%");
            app_data_dir.join("typst/packages/local")
        } else {
            panic!("Unsupported operating system")
        };

        // Ensure the directory exists
        assert!(path.is_dir(), "Typst local package directory does not exist");

        path
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_copy_dir() {
        let target_dir = std::path::PathBuf::from("/Users/isaac/Developer/rust-projects/xtyp/tmp2");
        match std::fs::remove_dir_all(&target_dir) {
            Ok(_) => {}
            Err(error) => {
                // Ignore the error if the directory does not exist
                assert!(matches!(error.kind(), std::io::ErrorKind::NotFound));
            }
        }

        // Copy the directory
        copy_dir::copy_dir("/Users/isaac/Developer/rust-projects/xtyp/tmp", &target_dir).unwrap();
    }
}
