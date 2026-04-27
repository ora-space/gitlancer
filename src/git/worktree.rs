use crate::domain::repo::Repository;
use crate::domain::worktree::WorktreeHandle;
use crate::error::{GitlancerError, ParseError};
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
    /// Resolves one named worktree once repository-discovery and worktree-indexing logic is implemented.
    pub fn resolve_worktree(
        &self,
        _request: ResolveWorktreeRequest<'_>,
    ) -> Result<WorktreeHandle, GitlancerError> {
        Err(ParseError::Unimplemented {
            feature: "resolve_worktree",
        }
        .into())
    }

    /// Finds which worktree contains a candidate path once canonical path matching is implemented.
    pub fn find_worktree(
        &self,
        _request: FindWorktreeRequest<'_>,
    ) -> Result<WorktreeHandle, GitlancerError> {
        Err(ParseError::Unimplemented {
            feature: "find_worktree",
        }
        .into())
    }
}
