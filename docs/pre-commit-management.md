## Pre-Commit Control Doc

Control pre-commit execution via `.pre-commit-config.yaml`:

- **Stages**: Limit when hook runs.  
  Options: `commit` (default), `push`, `prepare-commit-msg`, `commit-msg`, `post-commit`, `manual`.  
  Example:  
  ```yaml
  stages: [push]  # Runs only on git push
  ```

- **Exclude files**: Skip paths.  
  ```yaml
  exclude: '^docs/|.*\.txt$'  # Regex
  ```

- **Always run**: Override file matching.  
  ```yaml
  always_run: true  # Runs every time
  ```

- **Types/Files**: Run only on specific file types.  
  ```yaml
  types: [python]  # Or files: '^src/'
  ```

- **Require serial**: Run hooks one-by-one (for dependencies).

Per-hook control: Add keys under hook definition.

Global disable: `pre-commit uninstall`  
Temporary skip: `git commit --no-verify`  
Manual run: `pre-commit run --hook-stage manual`