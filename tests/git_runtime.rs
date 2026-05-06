mod common;

use std::path::Path;

use common::TestScaffold;
use gitlancer::git::branch::ListBranchesRequest;
use gitlancer::git::commit::{AddRequest, CommitRequest};
use gitlancer::git::repository::ListWorktreesRequest;
use gitlancer::git::status::StatusRequest;
use gitlancer::git::worktree::{FindWorktreeRequest, ResolveWorktreeRequest};
use gitlancer::{CliGitRunner, Git, RepoRelativePath, RepoRoot, WorktreeKind};

/// Creates an initial commit so linked worktrees can be created from a valid repository history.
fn seed_repository(scaffold: &TestScaffold) {
    scaffold
        .write_file(scaffold.repo_path(), "README.md", "seed repository\n")
        .expect("write seed file");
    scaffold
        .stage_all_and_commit("chore: seed repository")
        .expect("create initial commit");
}

/// Verifies the runtime can discover repositories, list worktrees, resolve linked worktrees, and enumerate branches.
#[test]
fn runtime_discovers_worktrees_and_branches() {
    let scaffold = TestScaffold::new("runtime-discovers-worktrees").expect("create scaffold");
    seed_repository(&scaffold);
    let linked_path = scaffold
        .create_linked_worktree("feature-tree", "feature/runtime")
        .expect("create linked worktree");

    let git = Git::new(CliGitRunner);
    let repository = git
        .discover_repository(RepoRoot::new(&linked_path))
        .expect("discover repository");
    let worktrees = git
        .list_worktrees(ListWorktreesRequest {
            repository: &repository,
        })
        .expect("list worktrees");
    let resolved = git
        .resolve_worktree(ResolveWorktreeRequest {
            repository: &repository,
            worktree_name: "feature-tree",
        })
        .expect("resolve linked worktree");
    let nested_path = linked_path.join("src").join("nested.txt");
    let found = git
        .find_worktree(FindWorktreeRequest {
            repository: &repository,
            candidate_path: &nested_path,
        })
        .expect("find worktree");
    let branches = git
        .list_branches(ListBranchesRequest {
            repository: &repository,
        })
        .expect("list branches");

    assert_eq!(
        worktrees.worktrees.len(),
        2,
        "main and linked worktrees should be visible"
    );
    assert!(
        worktrees
            .worktrees
            .iter()
            .any(|worktree| matches!(worktree.kind(), WorktreeKind::Main)),
        "one worktree should be classified as the main checkout"
    );
    assert!(
        matches!(resolved.kind(), WorktreeKind::Linked { name } if name == "feature-tree"),
        "the resolved worktree should match the linked worktree name"
    );
    assert_eq!(
        found.worktree_root().as_path(),
        linked_path.as_path(),
        "nested paths should resolve back to the owning linked worktree"
    );
    assert!(
        branches
            .branches
            .iter()
            .any(|branch| branch.as_str() == "main"),
        "the seeded repository should keep its main branch"
    );
    assert!(
        branches
            .branches
            .iter()
            .any(|branch| branch.as_str() == "feature/runtime"),
        "the linked worktree branch should be listed as a local branch"
    );
}

/// Verifies status, add, and commit flows return typed results when operating inside a linked worktree.
#[test]
fn runtime_reports_status_and_commit_metadata() {
    let scaffold = TestScaffold::new("runtime-status-and-commit").expect("create scaffold");
    seed_repository(&scaffold);
    let linked_path = scaffold
        .create_linked_worktree("feature-tree", "feature/runtime")
        .expect("create linked worktree");
    scaffold
        .write_file(&linked_path, "linked.txt", "linked worktree change\n")
        .expect("write linked file");

    let git = Git::new(CliGitRunner);
    let repository = git
        .discover_repository(RepoRoot::new(scaffold.repo_path()))
        .expect("discover repository");
    let worktree = git
        .resolve_worktree(ResolveWorktreeRequest {
            repository: &repository,
            worktree_name: "feature-tree",
        })
        .expect("resolve linked worktree");
    let status_before_add = git
        .status(StatusRequest {
            worktree: &worktree,
        })
        .expect("read worktree status before add");
    let add_result = git
        .add(AddRequest {
            worktree: &worktree,
            paths: vec![RepoRelativePath::new(Path::new("linked.txt"))],
        })
        .expect("stage linked file");
    let commit_result = git
        .commit(CommitRequest {
            worktree: &worktree,
            message: "feat: commit linked worktree change",
            allow_empty: false,
        })
        .expect("commit linked worktree change");

    assert!(
        status_before_add
            .entries
            .iter()
            .any(|entry| entry.raw.contains("linked.txt")),
        "status should include the untracked linked file before staging"
    );
    assert_eq!(
        add_result.staged_paths[0].as_path(),
        Path::new("linked.txt"),
        "the staged path should remain repo-relative"
    );
    assert_eq!(
        commit_result.summary, "feat: commit linked worktree change",
        "commit should return the latest summary"
    );
    assert_eq!(
        commit_result.commit_id.as_str().len(),
        40,
        "commit should return a full object ID"
    );
}
