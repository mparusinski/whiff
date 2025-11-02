use std::env;
use std::io;
// #[cfg(unix)]
// use std::os::unix;
#[cfg(windows)]
use std::os::windows;
use std::path::{Path, PathBuf};
use std::process;

use tempdir::TempDir;

pub struct TestEnv {
    /// Temporary working directory.
    temp_dir: TempDir,

    /// Path to the *whiff* executable.
    whiff_exe: PathBuf,
}

/// Create the working directory
fn create_working_directory() -> Result<TempDir, io::Error> {
    TempDir::new("whiff_tests")
}

/// Find the *whiff* executable
fn find_whiff_exe() -> PathBuf {
    let root = env::current_exe()
        .expect("Unable to find current executable")
        .parent()
        .expect("Test executable has no parent directory")
        .parent()
        .expect("whiff executable directory not found")
        .to_path_buf();

    let exe_name = if cfg!(windows) { "whiff.exe" } else { "whiff" };

    root.join(exe_name)
}

impl TestEnv {
    pub fn new() -> TestEnv {
        let temp_dir = create_working_directory().expect("Working directory");
        let whiff_exe = find_whiff_exe();

        TestEnv {
            temp_dir,
            whiff_exe,
        }
    }

    pub fn test_root(&self) -> PathBuf {
        self.temp_dir.path().to_path_buf()
    }

    pub fn test_exe(&self) -> &PathBuf {
        &self.whiff_exe
    }

    // TODO: Add arguments
    pub fn run_command(&self, path: &Path) -> process::Output {
        let normed_path = self.temp_dir.path().join(path);
        process::Command::new(&self.whiff_exe)
            .args([normed_path.to_str().expect("Invalid path")])
            .output()
            .expect("whiff output")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        let te = TestEnv::new();
        let output = te.run_command(Path::new("empty"));
        assert!(output.status.success());
    }
}
