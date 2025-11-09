use std::env;
use std::fs;
use std::io;
// #[cfg(unix)]
// use std::os::unix;
#[cfg(windows)]
use std::os::windows;
use std::path::{Path, PathBuf};
use std::process;
use std::time::SystemTime;

use tempdir::TempDir;

static JIFFY_MS: u128 = 100;

pub struct TestEnv {
    /// Test start
    start: SystemTime,

    /// Temporary working directory.
    temp_dir: TempDir,

    /// Path to the *whiff* executable.
    whiff_exe: PathBuf,
}

/// Format an error message for when *whiff* did not exit successfully.
fn format_exit_error(args: &[&str], output: &process::Output) -> String {
    format!(
        "`whiff {}` did not exit successfully.\nstdout:\n---\n{}---\nstderr:\n---\n{}---",
        args.join(" "),
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
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
        let start = SystemTime::now();
        let temp_dir = create_working_directory().expect("Working directory");
        let whiff_exe = find_whiff_exe();

        TestEnv {
            start,
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

    pub fn run_command(&self, path: &Path, args: &[&str]) -> process::Output {
        process::Command::new(&self.whiff_exe)
            .current_dir(self.temp_dir.path().join(path))
            .args(args)
            .output()
            .expect("whiff output")
    }

    pub fn assert_success_and_get_output(&self, args: &[&str]) -> process::Output {
        let output = self.run_command(Path::new("."), args);

        if !output.status.success() {
            panic!("{}", format_exit_error(args, &output));
        }

        output
    }

    pub fn assert_failure(&self, args: &[&str]) {
        let output = self.run_command(Path::new("."), args);

        if output.status.success() {
            panic!("Failure did not occur.");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_args_valid_path() {
        let te = TestEnv::new();
        let filename = "empty";
        te.assert_success_and_get_output(&[filename]);

        let target = te.test_root().join(filename);

        // Checking file exists
        assert!(fs::exists(target.clone()).expect("Unable to determine target file existence"));

        let metadata = fs::metadata(target.clone()).expect("Unable to access target file metadata");
        // Checking access timestamp are close enough as there is always
        // some delay between when the test starts and the file is created
        let access_time = metadata
            .accessed()
            .expect("Unable to fetch target file access time");
        let access_time_diff = access_time
            .duration_since(te.start)
            .expect("Unable to compute access time delta");
        assert!(access_time_diff.as_millis() < JIFFY_MS);

        let mod_time = metadata
            .accessed()
            .expect("Unable to fetch target file modification time");
        let mod_time_diff = mod_time
            .duration_since(te.start)
            .expect("Unable to compute modification time delta");
        assert!(mod_time_diff.as_millis() < JIFFY_MS);
    }

    #[test]
    fn test_no_args_invalid_path() {
        let te = TestEnv::new();
        let filename = Path::new("invalid")
            .join("empty")
            .to_string_lossy()
            .to_string();
        te.assert_failure(&[filename.as_str()]);
    }

    #[test]
    fn test_noargs_no_path() {
        let te = TestEnv::new();
        te.assert_failure(&[]);
    }
}
