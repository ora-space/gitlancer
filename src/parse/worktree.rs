use std::path::PathBuf;

use crate::domain::paths::{GitDir, RepoRoot, WorktreeRoot};
use crate::domain::worktree::{WorktreeHandle, WorktreeKind};
use crate::error::ParseError;

/// Parses `git worktree list --porcelain` output into typed worktree handles.
pub fn parse_worktree_list(stdout: &str) -> Result<Vec<WorktreeHandle>, ParseError> {
    let mut worktrees = Vec::new();
    let mut current_worktree: Option<PathBuf> = None;

    for line in stdout.lines() {
        let trimmed = line.trim();

        if let Some(path) = trimmed.strip_prefix("worktree ") {
            if let Some(worktree_root) = current_worktree.take() {
                worktrees.push(WorktreeHandle::new(
                    RepoRoot::new(worktree_root.clone()),
                    WorktreeRoot::new(worktree_root.clone()),
                    GitDir::new(worktree_root.join(".git")),
                    WorktreeKind::Main,
                ));
            }

            current_worktree = Some(PathBuf::from(path));
        }
    }

    if let Some(worktree_root) = current_worktree.take() {
        worktrees.push(WorktreeHandle::new(
            RepoRoot::new(worktree_root.clone()),
            WorktreeRoot::new(worktree_root.clone()),
            GitDir::new(worktree_root.join(".git")),
            WorktreeKind::Main,
        ));
    }

    if worktrees.is_empty() {
        return Err(ParseError::InvalidWorktreeList);
    }

    Ok(worktrees)
}
