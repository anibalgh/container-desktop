---
name: git
description: Git version control — branching, committing, merging, rebasing, stashing, log inspection, and conflict resolution. Use when the task involves git operations, reviewing history, staging changes, managing branches, or resolving merge conflicts.
---

# Git

## Status & History

```bash
git status                     # Working tree status
git diff                       # Unstaged changes
git diff --staged              # Staged changes
git log --oneline -20          # Recent commits
git log --graph --oneline --all
git blame <file>               # Line-by-line authorship
git show <commit>              # Show commit details + diff
```

## Branching

```bash
git branch                     # List local branches
git branch -a                  # List all branches (including remote)
git branch <name>              # Create branch
git checkout <branch>          # Switch branch
git checkout -b <name>         # Create and switch
git switch <branch>            # Modern alternative to checkout
git switch -c <name>           # Create and switch (modern)
git branch -d <name>           # Delete merged branch
git branch -D <name>           # Force delete branch
git push -u origin <branch>    # Push new branch + set upstream
```

## Staging & Committing

```bash
git add <file>                 # Stage file
git add -p                     # Interactive partial staging
git add -A                     # Stage all changes
git commit -m "message"        # Commit staged
git commit --amend             # Amend last commit
git commit --amend --no-edit   # Amend without changing message
```

## Merging & Rebasing

```bash
git merge <branch>             # Merge branch into current
git merge --abort              # Abort conflicted merge
git rebase <branch>            # Rebase current onto branch
git rebase -i HEAD~3           # Interactive rebase last 3 commits
git rebase --abort             # Abort rebase
git rebase --continue          # Continue after resolving conflicts
git cherry-pick <commit>       # Apply single commit
```

## Stashing

```bash
git stash                      # Stash uncommitted changes
git stash push -m "message"    # Stash with description
git stash list                 # List stashes
git stash pop                  # Apply and remove latest stash
git stash apply                # Apply without removing
git stash drop                 # Remove latest stash
```

## Remote Operations

```bash
git fetch origin               # Fetch without merging
git pull                       # Fetch + merge (or rebase if configured)
git pull --rebase              # Fetch + rebase
git push                       # Push commits
git push --force-with-lease    # Safer force push
git remote -v                  # List remotes
```

## Conflict Resolution

```bash
git diff --name-only --diff-filter=U  # List conflicted files
git checkout --theirs <file>          # Accept incoming version
git checkout --ours <file>            # Keep local version
```

After resolving conflicts: `git add <file>` then `git merge --continue` or `git rebase --continue`.

## Undoing

```bash
git reset HEAD <file>          # Unstage file
git checkout -- <file>         # Discard unstaged changes
git reset --soft HEAD~1        # Undo last commit (keep changes staged)
git reset --hard HEAD~1        # Undo last commit (discard changes)
git revert <commit>            # Revert commit (creates new commit)
```
