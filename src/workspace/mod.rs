//! Temporary filesystem layout for baseline and trial executions.
//!
//! The workspace keeps full source files only for the latest trial's A/B inputs
//! and the last accepted source. Historical trial records are stored as compact
//! diffs so long reduction runs do not accumulate thousands of full source
//! copies.

use std::{
    fs,
    path::{Path, PathBuf},
};

use tempfile::TempDir;

use crate::config::TrialSide;

/// Owns the temporary session directory.
#[derive(Debug)]
pub struct SessionWorkspace {
    root: PathBuf,
    temp_dir: Option<TempDir>,
    keep_temp: bool,
}

impl SessionWorkspace {
    /// Creates a new isolated workspace in the current directory.
    pub fn new(keep_temp: bool) -> anyhow::Result<Self> {
        let current_dir = std::env::current_dir()?;
        Self::new_in(&current_dir, keep_temp)
    }

    /// Creates a new isolated workspace next to the configured output file.
    pub fn for_output(output_path: &Path, keep_temp: bool) -> anyhow::Result<Self> {
        let base_dir = output_path
            .parent()
            .filter(|path| !path.as_os_str().is_empty())
            .unwrap_or_else(|| Path::new("."));
        Self::new_in(base_dir, keep_temp)
    }

    /// Creates a new isolated workspace under the provided base directory.
    pub fn new_in(base_dir: &Path, keep_temp: bool) -> anyhow::Result<Self> {
        let base_dir = base_dir.canonicalize().map_err(|error| {
            anyhow::anyhow!(
                "Failed to resolve workspace base directory '{}': {error}",
                base_dir.display()
            )
        })?;
        let temp_dir = tempfile::Builder::new()
            .prefix("code-minimizer-")
            .tempdir_in(&base_dir)
            .map_err(|error| {
                anyhow::anyhow!(
                    "Failed to create temporary workspace under '{}': {error}",
                    base_dir.display()
                )
            })?;
        let root = temp_dir.path().to_path_buf();

        fs::create_dir_all(root.join("accepted"))?;
        fs::create_dir_all(root.join("history"))?;
        fs::create_dir_all(root.join("trials"))?;

        Ok(Self {
            root,
            temp_dir: Some(temp_dir),
            keep_temp,
        })
    }

    /// Returns the root path of the session workspace.
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Returns the fixed directory reused for the current trial's full files.
    pub fn current_trial_dir(&self) -> PathBuf {
        self.root.join("trials").join("current")
    }

    /// Writes the current accepted source for crash inspection and debugging.
    pub fn write_accepted(&self, file_name: &str, source: &str) -> anyhow::Result<PathBuf> {
        let path = self.root.join("accepted").join(file_name);
        fs::write(&path, source)?;
        Ok(path)
    }

    /// Creates or refreshes the fixed current trial directory.
    ///
    /// The `trial_name` is recorded in `current.diff` and `history/*.diff`; it
    /// is not used as a directory name. This deliberately bounds full-file disk
    /// usage to one current A/B source pair plus the accepted source.
    pub fn prepare_trial(
        &self,
        trial_name: &str,
        file_name: &str,
        source: &str,
    ) -> anyhow::Result<TrialLayout> {
        let root = self.root.join("trials").join("current");
        if root.exists() {
            fs::remove_dir_all(&root)?;
        }
        fs::create_dir_all(&root)?;
        let previous = self.root.join("accepted").join(file_name);
        self.write_trial_diff(&root, trial_name, file_name, source, &previous)?;

        let mut sides = Vec::new();
        for side in [TrialSide::A, TrialSide::B] {
            let dir = root.join(side.as_dir_name());
            let output_dir = dir.join("out");
            fs::create_dir_all(&output_dir)?;
            let input_path = dir.join(file_name);
            fs::write(&input_path, source)?;
            sides.push(SideLayout {
                side,
                dir,
                input_path,
                output_dir,
            });
        }

        Ok(TrialLayout { root, sides })
    }

    /// Removes build outputs from the current trial while preserving current sources and diff.
    pub fn cleanup_trial_outputs(&self, layout: &TrialLayout) -> anyhow::Result<()> {
        for side in &layout.sides {
            for entry in fs::read_dir(&side.dir)? {
                let entry = entry?;
                let path = entry.path();
                if path == side.input_path {
                    continue;
                }
                remove_path(&path)?;
            }
        }
        Ok(())
    }

    /// Writes current and historical patches between accepted source and candidate.
    fn write_trial_diff(
        &self,
        trial_root: &Path,
        trial_name: &str,
        file_name: &str,
        source: &str,
        previous_path: &Path,
    ) -> anyhow::Result<()> {
        let previous = fs::read_to_string(previous_path).unwrap_or_default();
        let diff = unified_diff(&previous, source, file_name);
        let contents = format!("trial: {trial_name}\nfile: {file_name}\n\n{diff}");
        fs::write(trial_root.join("current.diff"), &contents)?;
        fs::write(
            self.root
                .join("history")
                .join(format!("{}.diff", safe_history_name(trial_name))),
            contents,
        )?;
        Ok(())
    }

    /// Persists the workspace directory when requested by `--keep-temp`.
    pub fn finish(mut self) -> anyhow::Result<Option<PathBuf>> {
        if self.keep_temp {
            let path = self.root.clone();
            if let Some(temp_dir) = self.temp_dir.take() {
                let _ = temp_dir.keep();
            }
            Ok(Some(path))
        } else {
            Ok(None)
        }
    }
}

/// Converts a trial label into a portable diff file stem.
fn safe_history_name(name: &str) -> String {
    let mut safe = String::with_capacity(name.len());
    for ch in name.chars() {
        if ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '.') {
            safe.push(ch);
        } else {
            safe.push('_');
        }
    }
    if safe.is_empty() {
        "trial".to_owned()
    } else {
        safe
    }
}

/// Removes either a file or a directory tree.
fn remove_path(path: &Path) -> anyhow::Result<()> {
    if path.is_dir() {
        fs::remove_dir_all(path)?;
    } else {
        fs::remove_file(path)?;
    }
    Ok(())
}

/// Builds a compact line-oriented unified diff.
fn unified_diff(previous: &str, current: &str, file_name: &str) -> String {
    if previous == current {
        return format!("--- accepted/{file_name}\n+++ current/{file_name}\n");
    }

    let previous_lines = previous.lines().collect::<Vec<_>>();
    let current_lines = current.lines().collect::<Vec<_>>();
    let mut prefix = 0_usize;
    while prefix < previous_lines.len()
        && prefix < current_lines.len()
        && previous_lines[prefix] == current_lines[prefix]
    {
        prefix += 1;
    }

    let mut previous_suffix = previous_lines.len();
    let mut current_suffix = current_lines.len();
    while previous_suffix > prefix
        && current_suffix > prefix
        && previous_lines[previous_suffix - 1] == current_lines[current_suffix - 1]
    {
        previous_suffix -= 1;
        current_suffix -= 1;
    }

    let context_before = prefix.saturating_sub(3);
    let previous_end = (previous_suffix + 3).min(previous_lines.len());
    let current_end = (current_suffix + 3).min(current_lines.len());

    let mut diff = String::new();
    diff.push_str(&format!("--- accepted/{file_name}\n"));
    diff.push_str(&format!("+++ current/{file_name}\n"));
    diff.push_str(&format!(
        "@@ -{},{} +{},{} @@\n",
        context_before + 1,
        previous_end.saturating_sub(context_before),
        context_before + 1,
        current_end.saturating_sub(context_before)
    ));

    for line in &previous_lines[context_before..prefix] {
        diff.push(' ');
        diff.push_str(line);
        diff.push('\n');
    }
    for line in &previous_lines[prefix..previous_suffix] {
        diff.push('-');
        diff.push_str(line);
        diff.push('\n');
    }
    for line in &current_lines[prefix..current_suffix] {
        diff.push('+');
        diff.push_str(line);
        diff.push('\n');
    }
    for line in &current_lines[current_suffix..current_end] {
        diff.push(' ');
        diff.push_str(line);
        diff.push('\n');
    }

    diff
}

/// Filesystem layout for a single baseline or trial attempt.
#[derive(Clone, Debug)]
pub struct TrialLayout {
    /// Root directory for this attempt.
    pub root: PathBuf,
    /// Side-specific layouts for A and B.
    pub sides: Vec<SideLayout>,
}

impl TrialLayout {
    /// Returns the side layout for A or B.
    pub fn side(&self, side: TrialSide) -> anyhow::Result<&SideLayout> {
        self.sides
            .iter()
            .find(|layout| layout.side == side)
            .ok_or_else(|| anyhow::anyhow!("Missing trial side {}", side.as_label()))
    }
}

/// Side-specific paths for one trial.
#[derive(Clone, Debug)]
pub struct SideLayout {
    /// Side identifier.
    pub side: TrialSide,
    /// Working directory used as process cwd.
    pub dir: PathBuf,
    /// Source input path passed through `{input}`.
    pub input_path: PathBuf,
    /// Output directory passed through `{output}`.
    pub output_dir: PathBuf,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn workspace_writes_trial_inputs_for_both_sides() {
        let workspace = SessionWorkspace::new(false).unwrap();
        let layout = workspace
            .prepare_trial("trial-1", "case.js", "console.log(1);")
            .unwrap();

        assert!(layout.side(TrialSide::A).unwrap().input_path.exists());
        assert!(layout.side(TrialSide::B).unwrap().input_path.exists());
    }

    #[test]
    fn workspace_can_be_created_next_to_output_file() {
        let dir = tempfile::tempdir().unwrap();
        let output = dir.path().join("case.min.js");

        let workspace = SessionWorkspace::for_output(&output, false).unwrap();

        assert!(workspace.root().starts_with(dir.path()));
        assert!(workspace.root().join("accepted").exists());
        assert!(workspace.root().join("history").exists());
        assert!(workspace.root().join("trials").exists());
    }

    #[test]
    fn workspace_reuses_current_trial_directory() {
        let workspace = SessionWorkspace::new(false).unwrap();
        let first = workspace
            .prepare_trial("trial-1", "case.js", "console.log(1);")
            .unwrap();
        let first_root = first.root.clone();
        let marker = first.root.join("stale");
        fs::write(&marker, "stale").unwrap();

        let second = workspace
            .prepare_trial("trial-2", "case.js", "console.log(2);")
            .unwrap();

        assert_eq!(first_root, second.root);
        assert!(!marker.exists());
        assert!(second.root.join("current.diff").exists());
        assert!(workspace.root.join("history/trial-1.diff").exists());
        assert!(workspace.root.join("history/trial-2.diff").exists());
        assert!(!workspace.root.join("trials/trial-1").exists());
        assert!(!workspace.root.join("trials/trial-2").exists());
    }

    #[test]
    fn workspace_cleans_trial_outputs_but_keeps_sources_and_diff() {
        let workspace = SessionWorkspace::new(false).unwrap();
        let layout = workspace.prepare_trial("trial-1", "case.js", "x").unwrap();
        let a = layout.side(TrialSide::A).unwrap();
        fs::write(a.dir.join("artifact.class"), "compiled").unwrap();
        fs::create_dir_all(a.output_dir.join("nested")).unwrap();

        workspace.cleanup_trial_outputs(&layout).unwrap();

        assert!(a.input_path.exists());
        assert!(layout.root.join("current.diff").exists());
        assert!(!a.dir.join("artifact.class").exists());
        assert!(!a.output_dir.exists());
    }
}
