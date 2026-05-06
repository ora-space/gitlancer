use crate::error::ParseError;

/// Marks worktree parsing as intentionally unimplemented until repository-aware semantics are ready.
pub fn parse_worktree_list(
    _stdout: &str,
) -> Result<Vec<crate::domain::worktree::WorktreeHandle>, ParseError> {
    Err(ParseError::Unimplemented {
        feature: "parse_worktree_list",
    })
}
