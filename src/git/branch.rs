use crate::domain::refs::BranchName;
use crate::domain::repo::Repository;
use crate::error::{GitlancerError, ParseError};
use crate::exec::runner::GitRunner;
use crate::git::Git;

/// Carries the information needed to read branch names from a repository.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListBranchesRequest<'a> {
    pub repository: &'a Repository,
}

/// Returns branch names in a typed form so upper layers can avoid raw-string plumbing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListBranchesResponse {
    pub branches: Vec<BranchName>,
}

impl<R: GitRunner> Git<R> {
    /// Lists branches once the branch parser and output contract are implemented.
    pub fn list_branches(
        &self,
        _request: ListBranchesRequest<'_>,
    ) -> Result<ListBranchesResponse, GitlancerError> {
        Err(ParseError::Unimplemented {
            feature: "list_branches",
        }
        .into())
    }
}
