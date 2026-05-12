# Copilot Instructions

Before any repository-specific analysis, answer, plan, review, or code change in this repository:

1. Run `./scripts/load-project-context.sh`.
2. Read every `SKILL.md` file printed by that command.
3. Treat `AGENTS.md` and those skill files as active project instructions for the rest of the session.
4. If the script fails, stop and report the failure instead of proceeding with partial context.
5. Re-run the script whenever `.deepseek/skills/` may have changed during the session.

This bootstrap step is mandatory for every session, regardless of model, tool, or agent implementation.
