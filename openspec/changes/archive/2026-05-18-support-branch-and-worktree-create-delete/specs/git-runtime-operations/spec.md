## ADDED Requirements

### Requirement: Runtime branch lifecycle commands are exposed through typed repository APIs
The runtime SHALL expose typed APIs to create and delete local branches from repository-aware inputs without requiring callers to assemble raw Git arguments.

#### Scenario: Creating a local branch
- **WHEN** a caller requests creation of a new local branch in a repository
- **THEN** the runtime creates that branch through the Git CLI
- **THEN** the response identifies the created branch as a `BranchName`

#### Scenario: Deleting a local branch
- **WHEN** a caller requests deletion of an existing local branch in a repository
- **THEN** the runtime deletes that branch through the Git CLI
- **THEN** the deleted branch no longer appears in `list_branches`

### Requirement: Runtime worktree lifecycle commands manage linked worktrees explicitly
The runtime SHALL expose typed APIs to create and delete linked worktrees while preserving the distinction between the main worktree and linked worktrees.

#### Scenario: Creating a linked worktree
- **WHEN** a caller requests creation of a linked worktree for a repository at a target checkout path
- **THEN** the runtime creates the linked worktree through the Git CLI
- **THEN** `list_worktrees` returns the new worktree as `WorktreeKind::Linked`

#### Scenario: Deleting a linked worktree
- **WHEN** a caller requests deletion of an existing linked worktree that belongs to a repository
- **THEN** the runtime removes that linked worktree through the Git CLI
- **THEN** `list_worktrees` no longer returns the removed worktree

### Requirement: Lifecycle mutations reject invalid destructive targets through typed errors
The runtime SHALL reject unsupported or mismatched lifecycle requests with typed validation errors before invoking Git whenever the invalid state can be determined from repository and worktree metadata.

#### Scenario: Rejecting deletion of the main worktree
- **WHEN** a caller requests deletion of the repository's main worktree
- **THEN** the runtime returns a domain validation error
- **THEN** no Git deletion command is invoked

#### Scenario: Rejecting removal of a worktree from another repository
- **WHEN** a caller requests deletion of a linked worktree that does not belong to the supplied repository
- **THEN** the runtime returns a domain validation error
- **THEN** no Git deletion command is invoked
