use crate::domain::paths::RepoRoot;
use crate::domain::repo::Repository;
use crate::error::{GitlancerError, ParseError};
use crate::exec::command::{GitCommand, GitIntent};
use crate::exec::env::GitEnv;
use crate::exec::runner::GitRunner;
use crate::git::Git;
use crate::git::worktree::{ListWorktreesResponse, ListWorktreesResult};
use crate::parse;

/// Carries the information needed to list worktrees for one repository.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListWorktreesRequest<'a> {
    pub repository: &'a Repository,
}

impl<R: GitRunner> Git<R> {
    /// Creates a repository handle from a validated root so use cases can accept repository identity explicitly.
    pub fn open_repository(&self, root: RepoRoot) -> Repository {
        Repository::new(root)
    }

    /// Lists worktrees for one repository using Git's porcelain worktree listing format.
    pub fn list_worktrees(
        &self,
        request: ListWorktreesRequest<'_>,
    ) -> Result<ListWorktreesResponse, GitlancerError> {
        let command = GitCommand::new(
            request.repository.root().as_path().to_path_buf(),
            vec![
                "worktree".to_string(),
                "list".to_string(),
                "--porcelain".to_string(),
            ],
            GitEnv::default(),
            GitIntent::ReadOnly,
        );
        let output = self.runner().run(&command)?;
        let worktrees = parse::worktree::parse_worktree_list(&output.stdout)?;

        Ok(ListWorktreesResult { worktrees }.into())
    }

    /// Discovers a repository from a candidate root path once path-validation logic is implemented.
    pub fn discover_repository(&self, _root: RepoRoot) -> Result<Repository, GitlancerError> {
        Err(ParseError::Unimplemented {
            feature: "discover_repository",
        }
        .into())
    }
}
