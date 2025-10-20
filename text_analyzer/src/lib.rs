use serde::Serialize;
use std::fs::Metadata;

// The command line expectation
pub struct Config {
    pub cmd: String,
    pub file_name: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough argument");
        }
        let cmd = args[1].clone();
        let file_name = args[2].clone();

        Ok(Config {
            cmd,
            file_name,
        })
    }
}

#[derive(Serialize)]
pub struct FileStats {
    pub name: String,
    pub file_type: String,
    pub size_bytes: u64,
    pub permissions: String,
}

impl FileStats {
    pub fn file_type(metadata: &Metadata) -> String {
        let file_type = if metadata.is_dir() {
            "directory"
        } else if metadata.is_file() {
            "file"
        } else if metadata.is_symlink() {
            "symlink"
        } else {
            "unknown"
        };
        file_type.to_string()
    }

    #[cfg(unix)]
    pub fn get_permissions(metadata: &Metadata) -> String {
        let permissions = {
            use std::os::unix::fs::PermissionsExt;
            format!("{:o}", metadata.permissions().mode() & 0o777)
        };
        permissions
    }

    #[cfg(not(unix))]
    pub fn get_permissions(metadata: &Metadata) -> String {
        let permissions = if metadata.mode().readonly() {
            "read-only".to_string()
        } else {
            "read-write".to_string()
        };
        permissions
    }
}
