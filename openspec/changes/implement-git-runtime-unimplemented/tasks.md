## 1. Parser Foundations

- [x] 1.1 Implement repository-aware worktree parsing for `git worktree list --porcelain`
- [x] 1.2 Implement commit metadata parsing helpers for commit ID and summary readback
- [x] 1.3 Implement porcelain-v2 status parsing into raw `StatusEntry` records

## 2. Runtime Behavior

- [x] 2.1 Update repository worktree listing to pass repository context into the parser
- [x] 2.2 Implement worktree resolution by name and nested path
- [x] 2.3 Implement local branch listing from machine-readable Git output
- [x] 2.4 Implement commit metadata readback in `Git::commit`

## 3. Verification

- [x] 3.1 Add parser unit tests for worktree, commit, and status decoding
- [x] 3.2 Add integration tests for linked-worktree listing, resolution, branch discovery, status, and commit flows
- [x] 3.3 Run formatting and test suites to verify the new runtime behavior
