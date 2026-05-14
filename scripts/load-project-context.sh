#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'EOF'
Usage: ./scripts/load-project-context.sh [--check]

Loads and verifies the project AI context from .deepseek/skills/*/SKILL.md.

Options:
  --check   Verify the bootstrap contract without printing every skill file.
EOF
}

script_dir=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
repo_root=$(cd -- "$script_dir/.." && pwd)
skills_root="$repo_root/.deepseek/skills"
mode="print"

if [[ $# -gt 1 ]]; then
  usage >&2
  exit 2
fi

if [[ ${1:-} == "--check" ]]; then
  mode="check"
elif [[ $# -eq 1 ]]; then
  usage >&2
  exit 2
fi

if [[ ! -d "$skills_root" ]]; then
  echo "error: skills directory not found: $skills_root" >&2
  exit 1
fi

expected_skills=(
  "docker"
  "docker-compose"
  "git"
  "github"
  "rust"
)

skill_dirs=()
while IFS= read -r skill_dir; do
  skill_dirs+=("$skill_dir")
done < <(find "$skills_root" -mindepth 1 -maxdepth 1 -type d | sort)

if [[ ${#skill_dirs[@]} -eq 0 ]]; then
  echo "error: no skill directories found under $skills_root" >&2
  exit 1
fi

for expected_skill in "${expected_skills[@]}"; do
  expected_file="$skills_root/$expected_skill/SKILL.md"
  if [[ ! -f "$expected_file" ]]; then
    echo "error: required skill file is missing: $expected_file" >&2
    exit 1
  fi
done

skill_files=()
skill_names=()

for skill_dir in "${skill_dirs[@]}"; do
  skill_name=$(basename "$skill_dir")
  skill_file="$skill_dir/SKILL.md"

  if [[ ! -f "$skill_file" ]]; then
    echo "error: missing SKILL.md for skill directory: $skill_dir" >&2
    exit 1
  fi

  skill_names+=("$skill_name")
  skill_files+=("$skill_file")
done

printf 'Project context bootstrap\n'
printf 'Repository: %s\n' "$repo_root"
printf 'Skills root: %s\n' "$skills_root"
printf 'Enumerated skills (%d):\n' "${#skill_names[@]}"
for skill_name in "${skill_names[@]}"; do
  printf ' - %s\n' "$skill_name"
done

if [[ "$mode" == "check" ]]; then
  printf 'Bootstrap check passed.\n'
  exit 0
fi

printf '\nReading skill files in repository order...\n'

for skill_file in "${skill_files[@]}"; do
  relative_path=${skill_file#"$repo_root"/}
  printf '\n===== BEGIN %s =====\n' "$relative_path"
  cat "$skill_file"
  printf '\n===== END %s =====\n' "$relative_path"
done
