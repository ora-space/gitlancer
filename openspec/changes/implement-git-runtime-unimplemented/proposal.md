## Why

The current v2 runtime exposes the right architectural boundaries, but several core Git flows still stop at `ParseError::Unimplemented`. That leaves Ora without usable repository discovery, worktree selection, branch enumeration, commit readback, or structured status data, which blocks real multi-worktree agent workflows.

## What Changes

- Implement repository-aware parsing and runtime behaviors for worktree listing, resolution, and path-based worktree lookup.
- Implement commit result readback so `Git::commit` returns a typed commit ID and summary.
- Implement structured status parsing for porcelain v2 output and branch listing for local branches.
- Add integration and parser-focused tests that cover linked worktrees, nested paths, branch discovery, and status/commit parsing.

## Capabilities

### New Capabilities
- `git-runtime-operations`: Provide working repository, worktree, branch, status, and commit flows for the v2 Git CLI runtime, including multi-worktree-aware behavior.

### Modified Capabilities

## Impact

- Affected code: `src/git/*`, `src/parse/*`, `src/domain/*`, `tests/common/*`, and new test modules under `tests/`.
- APIs: v2 runtime methods that currently return `Unimplemented` will become operational and return typed data.
- Dependencies: no new runtime dependencies are required; implementation stays on top of the Git CLI.
