use std::env;
use std::fs;
use std::io;
// #[cfg(unix)]
// use std::os::unix;
use chrono::{DateTime, Local, NaiveDate, TimeZone, Utc};
use std::fs::{File, FileTimes};
#[cfg(windows)]
use std::os::windows;
use std::path::{Path, PathBuf};
use std::process;
use std::time::SystemTime;
use time::UtcDateTime;

use tempdir::TempDir;

static JIFFY_MS: u128 = 100;
static EMPTY_FILE: &str = "empty";
static EXISTING_FILE: &str = "existing";

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

/// Convert SystemTime to chrono::DateTime
fn convert_systemtime_to_chrono(st: &SystemTime) -> DateTime<Utc> {
    let seconds = st.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
    DateTime::from_timestamp_secs(seconds.try_into().unwrap()).expect("Datetime conversion failed")
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

        let existing_filepath = temp_dir.path().join(Path::new(&EXISTING_FILE));
        let existing_fh =
            File::create(existing_filepath).expect("Unable to create existing file for test");
        let existing_ft = FileTimes::new()
            .set_accessed(SystemTime::UNIX_EPOCH)
            .set_modified(SystemTime::UNIX_EPOCH);
        existing_fh
            .set_times(existing_ft)
            .expect("Failure to set existing file times to UNIX_EPOCH");

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

    pub fn get_filepath(&self, file: &str) -> PathBuf {
        self.test_root().join(file)
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

    pub fn assert_access_time_touched(&self, metadata: &fs::Metadata) {
        let access_time = metadata
            .accessed()
            .expect("Unable to fetch target file access time");
        let access_time_diff = access_time
            .duration_since(self.start)
            .expect("Unable to compute access time delta");
        assert!(access_time_diff.as_millis() < JIFFY_MS);
    }

    pub fn assert_access_time_touched_with_datetime<Tz: TimeZone>(
        &self,
        metadata: &fs::Metadata,
        datetime: &DateTime<Tz>,
    ) {
        let access_time = metadata
            .accessed()
            .expect("Unable to fetch target file access time");

        assert_eq!(
            convert_systemtime_to_chrono(&access_time),
            datetime.to_utc().to_owned()
        );
    }

    pub fn assert_mod_time_touched(&self, metadata: &fs::Metadata) {
        let mod_time = metadata
            .accessed()
            .expect("Unable to fetch target file modification time");
        let mod_time_diff = mod_time
            .duration_since(self.start)
            .expect("Unable to compute modification time delta");
        assert!(mod_time_diff.as_millis() < JIFFY_MS);
    }

    pub fn assert_mod_time_touched_with_datetime<Tz: TimeZone>(
        &self,
        metadata: &fs::Metadata,
        datetime: &DateTime<Tz>,
    ) {
        let mod_time = metadata
            .accessed()
            .expect("Unable to fetch target file modification time");

        assert_eq!(
            convert_systemtime_to_chrono(&mod_time),
            datetime.to_utc().to_owned()
        );
    }

    pub fn assert_file_touched(&self, file: &str) {
        let filepath = self.get_filepath(file);
        assert!(fs::exists(filepath.clone()).expect("Unable to determine target file existence"));

        let metadata = fs::metadata(&filepath).expect("Unable to access target file metadata");
        // Checking access timestamp are close enough as there is always
        // some delay between when the test starts and the file is created
        self.assert_access_time_touched(&metadata);
        self.assert_mod_time_touched(&metadata);
    }

    pub fn assert_file_touched_with_date<Tz: TimeZone>(&self, file: &str, datetime: &DateTime<Tz>) {
        let filepath = self.get_filepath(file);
        let metadata = fs::metadata(&filepath).expect("Unable to access target file metadata");
        self.assert_access_time_touched_with_datetime(&metadata, datetime);
        self.assert_mod_time_touched_with_datetime(&metadata, datetime);
    }

    pub fn assert_file_non_existing(&self, file: &str) {
        let filepath = self.get_filepath(file);
        assert!(!filepath.exists());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_args_valid_path() {
        let te = TestEnv::new();
        let filename = EMPTY_FILE;
        te.assert_success_and_get_output(&[filename]);
        te.assert_file_touched(filename);
    }

    #[test]
    fn test_no_args_existing_file() {
        let te = TestEnv::new();
        let filename = EXISTING_FILE;
        te.assert_success_and_get_output(&[&filename]);
        te.assert_file_touched(&filename);
    }

    #[test]
    fn test_no_args_invalid_path() {
        let te = TestEnv::new();
        let filename = Path::new("invalid")
            .join(EMPTY_FILE)
            .to_string_lossy()
            .to_string();
        te.assert_failure(&[filename.as_str()]);
    }

    #[test]
    fn test_no_args_no_path() {
        let te = TestEnv::new();
        te.assert_failure(&[]);
    }

    #[test]
    fn test_no_create_valid_path() {
        let te = TestEnv::new();
        let filename = EMPTY_FILE;
        te.assert_success_and_get_output(&["-c", filename]);
        te.assert_file_non_existing(filename);
    }

    #[test]
    fn test_no_create_existing() {
        let te = TestEnv::new();
        let filename = EXISTING_FILE;
        te.assert_success_and_get_output(&["-c", filename]);
        te.assert_file_touched(&filename);
    }

    #[test]
    fn test_date_valid_path_ex1() {
        let te = TestEnv::new();
        let filename = EMPTY_FILE;
        te.assert_success_and_get_output(&["-d", "Sun, 29 Feb 2004 16:21:42  -0800", filename]);
        let expected_date = NaiveDate::from_ymd_opt(2004, 2, 29)
            .unwrap()
            .and_hms_opt(16, 21, 42)
            .unwrap();
        let expected_date = Local::now()
            .timezone()
            .from_local_datetime(&expected_date)
            .unwrap();
        te.assert_file_touched_with_date(filename, &expected_date);
    }

    #[test]
    fn test_date_valid_path_ex2() {
        let te = TestEnv::new();
        let filename = EMPTY_FILE;
        te.assert_success_and_get_output(&["-d", "2004-02-29 16:21:42", filename]);
        let expected_date = NaiveDate::from_ymd_opt(2004, 2, 29)
            .unwrap()
            .and_hms_opt(16, 21, 42)
            .unwrap();
        let expected_date = Local::now()
            .timezone()
            .from_local_datetime(&expected_date)
            .unwrap();
        te.assert_file_touched_with_date(filename, &expected_date);
    }

    #[test]
    fn test_date_valid_path_ex3() {
        let te = TestEnv::new();
        let filename = EMPTY_FILE;
        te.assert_success_and_get_output(&["-d", "next Thursday", filename]);

        let right_now = UtcDateTime::now();
        let next_thursday = right_now.date().next_occurrence(time::Weekday::Thursday);
        let expected_date = right_now.replace_date(next_thursday);
        let expected_date = DateTime::from_timestamp(expected_date.unix_timestamp(), 0).unwrap();
        te.assert_file_touched_with_date(filename, &expected_date);
    }
}
