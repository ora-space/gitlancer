use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

/// Creates an isolated on-disk git repository for integration tests.
pub struct TestScaffold {
    tmp_root: PathBuf,
    repo_path: PathBuf,
}

impl TestScaffold {
    /// Creates a unique sandbox repository under `.tmp` and initializes git.
    pub fn new(test_name: &str) -> Result<Self, String> {
        let project_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let tmp_root = project_root.join(".tmp");
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| format!("system time error: {e}"))?
            .as_nanos();
        let unique = format!("{test_name}-{}-{ts}", std::process::id());
        let repo_path = tmp_root.join(unique);

        fs::create_dir_all(&repo_path).map_err(|e| format!("create sandbox failed: {e}"))?;

        run_git(&repo_path, ["init", "."])?;
        run_git(&repo_path, ["config", "user.name", "gitlancer-test"])?;
        run_git(
            &repo_path,
            ["config", "user.email", "gitlancer-test@example.com"],
        )?;

        Ok(Self {
            tmp_root,
            repo_path,
        })
    }

    /// Returns the initialized repository absolute path.
    pub fn repo_path(&self) -> &Path {
        &self.repo_path
    }

    /// Returns the root `.tmp` path used by test sandboxes.
    pub fn tmp_root(&self) -> &Path {
        &self.tmp_root
    }

    /// Runs raw git arguments inside this sandbox repository.
    pub fn run_git<I, S>(&self, args: I) -> Result<String, String>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        run_git(&self.repo_path, args)
    }
}

impl Drop for TestScaffold {
    /// Deletes the entire `.tmp` tree so no test artifacts survive test execution.
    fn drop(&mut self) {
        if self.tmp_root.exists() {
            let _ = fs::remove_dir_all(&self.tmp_root);
        }
    }
}

/// Runs a git command in a target directory and returns stdout when successful.
fn run_git<I, S>(cwd: &Path, args: I) -> Result<String, String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let output = Command::new("git")
        .current_dir(cwd)
        .env("GIT_TERMINAL_PROMPT", "0")
        .env("LANG", "C")
        .env("GIT_PAGER", "cat")
        .args(args)
        .output()
        .map_err(|e| format!("spawn git failed: {e}"))?;

    if output.status.success() {
        return String::from_utf8(output.stdout).map_err(|e| format!("stdout utf8 error: {e}"));
    }

    Err(format!(
        "git failed with code {:?}: {}",
        output.status.code(),
        String::from_utf8_lossy(&output.stderr)
    ))
}
