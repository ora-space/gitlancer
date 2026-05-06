## Context

The repository already has the v2 module boundaries in place: domain types model repositories and worktrees, the execution layer owns Git CLI invocation, and the `git` layer exposes typed use cases. The missing piece is behavior. Several core methods currently terminate at `ParseError::Unimplemented`, which means the runtime shape is present but the system still cannot drive real Ora workflows such as discovering linked worktrees, selecting the correct checkout for a file, reading local branches, parsing status, or returning commit metadata after `git commit`.

The implementation needs to stay on the Git CLI, preserve the existing generic `Git<R: GitRunner>` shape, and keep multi-worktree semantics explicit. Tests should exercise both parser logic and real Git integration behavior so future refactors can safely evolve the runtime.

## Goals / Non-Goals

**Goals:**
- Implement worktree parsing from `git worktree list --porcelain` with correct `WorktreeKind`, repository ownership, and linked-worktree naming.
- Implement repository-level worktree lookup APIs that can resolve a worktree by name and by arbitrary nested filesystem path.
- Implement commit metadata readback, structured status parsing, and local branch enumeration.
- Add tests that lock in parser behavior and multi-worktree integration behavior.

**Non-Goals:**
- Add support for remote/network Git operations.
- Replace the Git CLI with `libgit2` or another backend.
- Fully model every field in porcelain v2 status output; this change only needs enough structure for the current `StatusEntry` type.

## Decisions

### Use repository-aware parser inputs for worktrees
Worktree parsing cannot infer repository ownership from each record alone, especially for linked worktrees. The parser will therefore accept the repository root as input and stamp every parsed `WorktreeHandle` with the same `RepoRoot`. A worktree will be classified as `Main` when its worktree root equals the repository root; otherwise it becomes `Linked`.

Alternative considered:
- Keep the existing parser signature and guess repository identity from each worktree path.
Why not:
- That produces incorrect `repo_root` values for linked worktrees and silently breaks cross-worktree comparisons.

### Derive linked worktree names from checkout paths
`ResolveWorktreeRequest` currently addresses worktrees by a human-readable name. The implementation will derive that name from the final path segment of each linked worktree root, because it is stable in the current test scaffolding and aligns with how linked worktrees are created in repository fixtures.

Alternative considered:
- Derive names from `branch` fields in porcelain output.
Why not:
- Worktree identity and branch identity are related but not equivalent, and detached worktrees would still need a path-based fallback.

### Keep branch and status parsing intentionally small but deterministic
For branches, the runtime will list local branches with a machine-readable `for-each-ref` format and parse the branch names into `BranchName`. For status, the parser will read `git status --porcelain=v2 -z` and preserve each entry as a raw status record inside `StatusEntry`.

Alternative considered:
- Fully model every porcelain v2 field immediately.
Why not:
- The current public type only exposes `raw`, so full modeling would add complexity without changing the contract.

### Read commit metadata through follow-up plumbing commands
After a successful `git commit`, the runtime will read `HEAD` and the latest summary using `git rev-parse HEAD` and `git log -1 --pretty=%s HEAD`. This keeps the commit API deterministic and avoids parsing human-facing commit output.

Alternative considered:
- Parse stdout from `git commit`.
Why not:
- Commit output can vary with hooks, locale, and Git formatting, while plumbing/log commands are more stable.

## Risks / Trade-offs

- [Linked worktree naming depends on path segments] → Mitigation: keep the rule explicit in code and tests so a later rename strategy can evolve without ambiguity.
- [Status entries remain minimally structured] → Mitigation: parse machine-readable record boundaries correctly now, and leave room to enrich `StatusEntry` later without reworking command execution.
- [Repository discovery is still shallow] → Mitigation: implement enough for the current runtime methods and keep path validation logic localized so a deeper discovery API can extend it later.

## Migration Plan

- Implement parser helpers first so use-case methods can compose on stable decoding behavior.
- Update repository/worktree/branch/commit methods to use the new parsers and command flows.
- Add parser unit tests and integration tests using linked worktrees.
- Run `cargo fmt` and `cargo test` to validate the runtime before further feature work.

## Open Questions

- Whether `StatusEntry` should stay raw or become a richer enum once Ora starts consuming structured change types.
- Whether repository discovery should eventually accept arbitrary nested paths and canonicalize to the true Git toplevel instead of requiring a validated `RepoRoot` wrapper.
