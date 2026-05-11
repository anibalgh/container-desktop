---
name: github
description: GitHub CLI (gh) for pull requests, issues, releases, and repository management. Use when creating/reviewing PRs, managing issues, checking CI status, viewing workflow runs, or interacting with GitHub repositories via the gh command.
---

# GitHub CLI

## Authentication

```bash
gh auth status                 # Check login status
gh auth login                  # Authenticate
```

## Pull Requests

```bash
gh pr list                     # List open PRs
gh pr list --state merged      # List merged PRs
gh pr view                     # View current branch PR
gh pr view <number>            # View specific PR
gh pr view --web               # Open PR in browser
gh pr create                   # Create PR (interactive)
gh pr create --title "..." --body "..."
gh pr create --draft           # Create draft PR
gh pr checkout <number>        # Checkout PR branch locally
gh pr merge <number>           # Merge PR
gh pr merge --squash           # Squash merge
gh pr merge --rebase           # Rebase merge
gh pr review --approve         # Approve PR
gh pr review --comment -b "..." # Comment on PR
gh pr review --request-changes -b "..."
gh pr close <number>           # Close PR without merging
gh pr diff <number>            # View PR diff
gh pr checks <number>          # View CI checks
```

## Issues

```bash
gh issue list                  # List open issues
gh issue list --label "bug"    # Filter by label
gh issue view <number>         # View issue
gh issue view --web            # Open in browser
gh issue create                # Create issue (interactive)
gh issue create --title "..." --body "..."
gh issue close <number>        # Close issue
gh issue comment <number> -b "..."
gh issue reopen <number>
```

## Repository

```bash
gh repo view                   # View current repo
gh repo view --web             # Open in browser
gh repo clone <owner/repo>     # Clone repo
gh repo fork                   # Fork current repo
gh repo create <name>          # Create new repo
```

## Workflows & Actions

```bash
gh run list                    # List recent workflow runs
gh run view <id>               # View run details
gh run watch <id>              # Watch run progress
gh run rerun <id>              # Rerun failed workflow
gh workflow list               # List workflows
gh workflow run <name>         # Trigger workflow
```

## Releases

```bash
gh release list                # List releases
gh release view <tag>          # View release
gh release create <tag>        # Create release (interactive)
gh release create <tag> --title "..." --notes "..."
gh release upload <tag> <file> # Upload asset to release
```

## Common Workflows

**Create PR from current branch:**
```bash
git push -u origin HEAD
gh pr create --title "..." --body "..."
```

**Review and merge a PR:**
```bash
gh pr checkout <number>
# ... test locally ...
gh pr review <number> --approve
gh pr merge <number> --squash
```

**Create issue from bug report:**
```bash
gh issue create --title "Bug: ..." --label "bug" --body "$(cat bug_report.md)"
```

## gh Tips

- Use `--json` flag for programmatic output: `gh pr list --json number,title,state`
- `gh api` for direct GitHub API calls: `gh api /repos/owner/repo/issues`
- `gh` respects `.gitconfig` and GitHub token from `gh auth`
