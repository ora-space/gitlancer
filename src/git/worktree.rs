use crate::domain::repo::Repository;
use crate::domain::worktree::WorktreeHandle;
use crate::error::{DomainError, GitlancerError};
use crate::exec::runner::GitRunner;
use crate::git::Git;

/// Carries the information needed to select one worktree from a repository.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolveWorktreeRequest<'a> {
    pub repository: &'a Repository,
    pub worktree_name: &'a str,
}

/// Carries the information needed to locate which worktree contains a caller path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FindWorktreeRequest<'a> {
    pub repository: &'a Repository,
    pub candidate_path: &'a std::path::Path,
}

/// Returns the complete list of worktrees associated with one repository.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListWorktreesResponse {
    pub worktrees: Vec<WorktreeHandle>,
}

/// Represents the internal result shape produced before it is wrapped for the public response type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ListWorktreesResult {
    pub worktrees: Vec<WorktreeHandle>,
}

impl From<ListWorktreesResult> for ListWorktreesResponse {
    /// Converts the internal result shape into the stable public response type.
    fn from(value: ListWorktreesResult) -> Self {
        Self {
            worktrees: value.worktrees,
        }
    }
}

impl<R: GitRunner> Git<R> {
    /// Resolves one named worktree by scanning the repository's known worktrees and matching their stable names.
    pub fn resolve_worktree(
        &self,
        request: ResolveWorktreeRequest<'_>,
    ) -> Result<WorktreeHandle, GitlancerError> {
        let worktrees = self
            .list_worktrees(crate::git::repository::ListWorktreesRequest {
                repository: request.repository,
            })?
            .worktrees;

        worktrees
            .into_iter()
            .find(|worktree| worktree_name(worktree) == request.worktree_name)
            .ok_or_else(|| {
                GitlancerError::Domain(DomainError::NotAWorktree(
                    request.repository.root().as_path().to_path_buf(),
                ))
            })
    }

    /// Finds which worktree contains a candidate path by choosing the deepest worktree root prefix match.
    pub fn find_worktree(
        &self,
        request: FindWorktreeRequest<'_>,
    ) -> Result<WorktreeHandle, GitlancerError> {
        let candidate = normalize_candidate_path(request.candidate_path);
        let worktrees = self
            .list_worktrees(crate::git::repository::ListWorktreesRequest {
                repository: request.repository,
            })?
            .worktrees;

        worktrees
            .into_iter()
            .filter(|worktree| candidate.starts_with(worktree.worktree_root().as_path()))
            .max_by_key(|worktree| worktree.worktree_root().as_path().components().count())
            .ok_or_else(|| GitlancerError::Domain(DomainError::NotAWorktree(candidate)))
    }
}

/// Derives the stable name callers use to address one worktree.
fn worktree_name(worktree: &WorktreeHandle) -> &str {
    match worktree.kind() {
        crate::domain::worktree::WorktreeKind::Main => "main",
        crate::domain::worktree::WorktreeKind::Linked { name } => name.as_str(),
    }
}

/// Normalizes a candidate path for prefix comparisons while preserving non-existent nested paths.
fn normalize_candidate_path(path: &std::path::Path) -> std::path::PathBuf {
    std::fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf())
}
