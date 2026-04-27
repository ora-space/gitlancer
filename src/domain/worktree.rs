use crate::domain::paths::{GitDir, RepoRoot, WorktreeRoot};

/// Distinguishes the main checkout from linked worktrees because they have different lifecycle semantics.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorktreeKind {
    Main,
    Linked { name: String },
}

/// Represents one executable worktree context that belongs to a repository.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorktreeHandle {
    repo_root: RepoRoot,
    worktree_root: WorktreeRoot,
    git_dir: GitDir,
    kind: WorktreeKind,
}

impl WorktreeHandle {
    /// Creates a worktree handle from validated repository and worktree metadata.
    pub fn new(
        repo_root: RepoRoot,
        worktree_root: WorktreeRoot,
        git_dir: GitDir,
        kind: WorktreeKind,
    ) -> Self {
        Self {
            repo_root,
            worktree_root,
            git_dir,
            kind,
        }
    }

    /// Returns the repository root that owns this worktree.
    pub fn repo_root(&self) -> &RepoRoot {
        &self.repo_root
    }

    /// Returns the checkout root where worktree-scoped Git commands should execute.
    pub fn worktree_root(&self) -> &WorktreeRoot {
        &self.worktree_root
    }

    /// Returns the gitdir backing this worktree so linked worktrees can be handled explicitly.
    pub fn git_dir(&self) -> &GitDir {
        &self.git_dir
    }

    /// Returns the worktree kind so callers can branch on main versus linked behavior deliberately.
    pub fn kind(&self) -> &WorktreeKind {
        &self.kind
    }
}
