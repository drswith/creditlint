# Enforcement Rollout

Use this skill when a user wants to roll out `creditlint` in another repository
and cares about strong, practical interception rather than only installing the
binary.

## Purpose

Help the user deploy `creditlint` across the full governance path:

- local commit creation
- pushed commit ranges in CI
- pull request title and body text
- final squash or merge commit message
- repository settings that enforce the chosen controls

The skill must keep one boundary explicit:

- local hooks and CI checks do not fully control a final squash merge message
  edited in the hosting platform UI

## What "complete interception" means

For most teams, complete interception means all of the following are covered:

1. Local developer commits are checked before commit creation.
2. Commits pushed by cloud agents, CI-generated commits, or bypassed local hooks
   are checked again in CI.
3. Pull request title/body text is checked if the platform may use it to form a
   final squash message.
4. The final protected-branch commit message is checked at the platform
   boundary.
5. The protected branch requires these controls instead of treating them as
   optional lint.

If any of those layers is missing, explain what gap remains.

## Rollout Checklist

Apply this checklist to the target repository.

### 1. Local hook path

- Run `creditlint init` in the target repository unless a policy file already
  exists.
- Run `creditlint install-hook`, or integrate
  `creditlint check --message-file "$1"` into the repository's existing
  `commit-msg` hook.
- Confirm the repository does not silently replace or bypass that hook in local
  tooling.

Why:

- This is the fastest feedback path for normal developer commits.

### 2. CI commit path

- Add a required CI check that runs `creditlint check --range <base>..HEAD` for
  pull requests.
- Use full history checkout when range resolution depends on merge base history.
- Add `creditlint audit --all` or an equivalent full-history audit on protected
  branch pushes when the repository wants ongoing baseline scanning.

Why:

- This catches commits created by cloud agents, bots, rebases, or users whose
  local hook never ran.

### 3. PR title and body path

- If squash merge is enabled, validate PR title and body text because hosting
  platforms can reuse that text in the final squash commit message.
- In CI, write the PR title/body into a temporary file and run
  `creditlint check --message-file <file>`.

Why:

- Commit-range checks do not see text that only exists in the PR UI.

### 4. Final squash or merge message path

- If the active policy can be safely represented by one GitHub ruleset regex,
  use `creditlint github ruleset-pattern` and configure a repository or branch
  ruleset for commit-message restriction.
- If the active policy cannot be safely represented as one ruleset regex, use a
  merge bot or equivalent controlled merge path that passes the final message to
  `creditlint check --message-file`.

Why:

- This is the control layer for the final platform-generated commit message.

### 5. Protected-branch governance

- Make the CI check required on the protected branch.
- Make the commit-message ruleset or merge-bot path mandatory for branches where
  squash merge remains enabled.
- Restrict direct pushes if the team expects all changes to pass through PR
  validation.
- Document who owns policy changes and repository settings changes.

Why:

- Without required enforcement, `creditlint` becomes advisory instead of
  blocking.

## Output Format

When using this skill for a target repository, produce:

1. A gap analysis:
   - what is already covered
   - what is missing
   - what risk each missing layer leaves behind
2. A concrete rollout plan:
   - exact files or workflows to modify
   - exact GitHub settings or repository controls to add
   - whether rulesets are enough or a merge bot is still required
3. A final enforcement summary:
   - what becomes blocked locally
   - what becomes blocked in CI
   - what still depends on repository settings outside the repo

## Guardrails

- Do not claim CI alone fully protects final squash messages.
- Do not claim local hooks protect cloud agents or platform-generated commits.
- If the platform is not GitHub, map the same layers onto that platform's
  equivalent controls instead of pretending GitHub-specific steps apply
  unchanged.
- If the policy cannot be safely exported as one GitHub ruleset regex, say that
  clearly and route the repository toward merge-bot validation.
