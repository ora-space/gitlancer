## 1. Branch Lifecycle API

- [x] 1.1 Add typed request/response models and any lifecycle enums needed for branch creation and deletion in `src/git/branch.rs`.
- [x] 1.2 Implement branch create/delete Git command assembly and result shaping in `src/git/branch.rs`.
- [x] 1.3 Add command-level tests that verify branch lifecycle arguments and intents with the injected runner.

## 2. Worktree Lifecycle API

- [x] 2.1 Add typed request/response models and any lifecycle enums needed for linked worktree creation and deletion in `src/git/worktree.rs`.
- [x] 2.2 Implement linked-worktree create/delete flows, including repository ownership checks and main-worktree deletion rejection.
- [x] 2.3 Extend domain errors only where lifecycle validation needs clearer typed failures.

## 3. Verification And Docs

- [x] 3.1 Add real Git integration tests that verify branch creation/deletion and linked-worktree creation/deletion through the public runtime APIs.
- [x] 3.2 Add integration coverage for rejected destructive operations, including deleting the main worktree and deleting a worktree from another repository.
- [x] 3.3 Update `docs/` documentation that describes the public runtime surface if the new lifecycle APIs change the documented architecture.
