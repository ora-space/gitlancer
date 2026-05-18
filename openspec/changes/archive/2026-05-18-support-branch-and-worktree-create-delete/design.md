## Context

Gitlancer already exposes typed repository, branch, and worktree inspection APIs, but callers still need raw Git commands when they want to create or remove local branches or linked worktrees. The current architecture intentionally separates domain validation, Git command assembly, and parsing, so lifecycle support should extend the existing `git` modules and error model rather than introducing an untyped escape hatch.

The change crosses multiple runtime surfaces:

- `src/git/branch.rs` currently provides read-only branch listing.
- `src/git/worktree.rs` currently provides worktree resolution and path selection.
- Domain and error types currently cover repository/worktree ownership and path safety, but not destructive lifecycle preconditions such as removing the main worktree or deleting an in-use branch.

## Goals / Non-Goals

**Goals:**

- Add typed request/response APIs for branch creation and deletion.
- Add typed request/response APIs for linked worktree creation and deletion.
- Reuse repository- and worktree-aware domain types so callsites cannot pass ambiguous raw paths.
- Preserve the existing static-dispatch runtime design and testability through `Git<R: GitRunner>`.
- Surface validation failures through explicit domain errors whenever Git preconditions can be checked before execution.

**Non-Goals:**

- Supporting remote branch management or networked branch creation.
- Supporting deletion of the repository's main worktree.
- Adding force-style behavior through bare booleans; any destructive variants should use explicit request shapes or enums.
- Redesigning the broader execution or parser architecture.

## Decisions

Use dedicated lifecycle request/response types in the existing branch and worktree modules.
Why: The codebase already favors explicit typed boundaries, and branch/worktree lifecycle operations belong next to the read/query flows that manipulate the same concepts.
Alternative considered: a generic "run Git mutation" API. Rejected because it would bypass the domain model and make callsites harder to validate and test.

Model destructive variants with explicit enums instead of booleans.
Why: The repository conventions explicitly discourage opaque boolean parameters. Branch deletion and worktree removal both have meaningful modes, so enum-backed request fields keep callsites self-documenting.
Alternative considered: methods such as `delete_branch(force: bool)`. Rejected because it hides destructive intent behind positional literals.

Keep command construction on top of stable Git CLI entry points and avoid parsing human-oriented success output.
Why: The existing runtime treats Git as the source of truth and already relies on machine-oriented commands. Lifecycle methods can often return typed handles or branch names assembled from validated inputs without scraping Git's display strings.
Alternative considered: parse stdout from `git branch` or `git worktree add/remove`. Rejected because those messages are presentation-oriented and less stable than the command side effects plus existing query APIs.

Validate repository/worktree invariants before invoking destructive commands when the runtime already has enough information.
Why: We can cheaply reject impossible operations such as deleting the main worktree, removing a worktree from the wrong repository, or issuing branch deletion against an unknown local branch. That keeps failures consistent with the existing `DomainError` layer.
Alternative considered: delegate all validation to Git and surface only execution failures. Rejected because it weakens the typed runtime contract and makes tests depend on CLI stderr wording.

Verify post-mutation state through existing query APIs in integration tests instead of adding special-case parsers.
Why: The current design already has typed list/resolve methods that can confirm branch/worktree creation and deletion outcomes. Reusing them keeps the implementation smaller and exercises realistic end-to-end behavior.
Alternative considered: introduce dedicated parsers for mutation stdout. Rejected because they add maintenance cost without improving the API contract.

## Risks / Trade-offs

[Git preconditions vary across commands] -> Mitigation: validate what the runtime can know locally, and let command-specific Git failures continue through `GitExecError::NonZeroExit` when the CLI owns the final rule.

[Too much branch and worktree option growth could bloat a single module] -> Mitigation: keep request/response types operation-specific, and split helper modules if either file approaches the repository size limits.

[Deleting resources is inherently more dangerous than reading them] -> Mitigation: represent deletion modes explicitly, reject unsupported targets like the main worktree up front, and cover destructive paths with real Git integration tests.

## Migration Plan

This change is additive at the crate API level, so no repository migration is required.

Implementation rollout:

1. Add request/response models and supporting enums for branch/worktree lifecycle methods.
2. Implement branch lifecycle commands in `src/git/branch.rs`.
3. Implement linked-worktree lifecycle commands in `src/git/worktree.rs`.
4. Extend domain errors only where pre-execution invariants need clearer failure modes.
5. Add or update architecture/docs content if the public runtime surface changes materially.
6. Add fake-runner and real Git integration coverage for successful and rejected lifecycle operations.

Rollback strategy:

- Revert the new lifecycle APIs and tests as one change if downstream integration reveals unsafe semantics.

## Open Questions

- Whether branch creation should support optional start points in the first iteration or begin with `HEAD`-based creation only.
- Whether worktree creation should allow detached checkouts immediately or reserve that for a follow-up change once the base lifecycle path is stable.
