## ADDED Requirements

### Requirement: Repository worktree queries return multi-worktree-aware handles
The runtime SHALL list repository worktrees from `git worktree list --porcelain` and return `WorktreeHandle` values whose `repo_root` points to the owning repository, whose `worktree_root` points to the checkout root, and whose `kind` distinguishes the main worktree from linked worktrees.

#### Scenario: Listing main and linked worktrees
- **WHEN** a repository contains its main checkout and one linked worktree
- **THEN** `list_worktrees` returns two worktrees
- **THEN** exactly one returned worktree is `WorktreeKind::Main`
- **THEN** the linked worktree is returned as `WorktreeKind::Linked`

### Requirement: Worktrees can be resolved by name and by nested path
The runtime SHALL resolve linked worktrees by their configured worktree name and SHALL locate which worktree contains an arbitrary nested filesystem path.

#### Scenario: Resolving a linked worktree by name
- **WHEN** a caller requests a linked worktree name that exists in the repository
- **THEN** `resolve_worktree` returns the corresponding `WorktreeHandle`

#### Scenario: Finding a worktree from a nested file path
- **WHEN** a caller provides a path nested under a linked worktree checkout
- **THEN** `find_worktree` returns the linked worktree that contains that path

### Requirement: Runtime inspection commands return typed branch, status, and commit data
The runtime SHALL expose local branches, status entries, and commit metadata through typed responses backed by machine-readable Git output.

#### Scenario: Listing local branches
- **WHEN** a repository contains multiple local branches
- **THEN** `list_branches` returns each local branch as a `BranchName`

#### Scenario: Reading structured status data
- **WHEN** a worktree contains tracked or untracked changes
- **THEN** `status` returns at least one `StatusEntry` for each porcelain-v2 status record

#### Scenario: Reading commit metadata after a successful commit
- **WHEN** `commit` succeeds in a worktree
- **THEN** the response includes the `HEAD` commit ID
- **THEN** the response includes the latest commit summary
