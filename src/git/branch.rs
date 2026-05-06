use crate::domain::refs::BranchName;
use crate::domain::repo::Repository;
use crate::error::GitlancerError;
use crate::exec::command::{GitCommand, GitIntent};
use crate::exec::env::GitEnv;
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
    /// Lists local branches from Git's ref database so callers can avoid parsing human-oriented branch output.
    pub fn list_branches(
        &self,
        request: ListBranchesRequest<'_>,
    ) -> Result<ListBranchesResponse, GitlancerError> {
        let command = GitCommand::new(
            request.repository.root().as_path().to_path_buf(),
            vec![
                "for-each-ref".to_string(),
                "--format=%(refname:short)".to_string(),
                "refs/heads".to_string(),
            ],
            GitEnv::default(),
            GitIntent::ReadOnly,
        );
        let output = self.runner().run(&command)?;
        let branches = output
            .stdout
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(|line| BranchName::new(line.to_string()))
            .collect();

        Ok(ListBranchesResponse { branches })
    }
}
