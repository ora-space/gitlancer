## Why

Gitlancer can currently inspect repositories, branches, and worktrees, but it cannot perform the lifecycle operations that callers need to create or remove those resources. Adding typed branch and worktree create/delete flows now closes a core gap in the Git runtime surface and lets higher layers automate repository setup and cleanup without dropping down to ad-hoc shell commands.

## What Changes

- Add typed runtime APIs to create and delete local branches from repository-aware inputs.
- Add typed runtime APIs to create and delete linked worktrees from repository-aware inputs.
- Define explicit request and response models for lifecycle operations so callsites do not rely on ambiguous booleans or raw strings.
- Validate branch/worktree ownership and destructive preconditions before invoking Git so errors stay domain-oriented.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `git-runtime-operations`: Extend the runtime capability from read-only branch/worktree inspection to include typed branch and linked-worktree lifecycle operations.

## Impact

- Affected code: `src/git/branch.rs`, `src/git/worktree.rs`, shared domain types, parsing/execution support, and integration tests around real Git repositories.
- Affected APIs: public Rust runtime methods for branch and worktree operations.
- Dependencies/systems: no new external dependencies expected; uses existing Git CLI execution flow with additional lifecycle commands.
