## Git Submodules Management Doc

### Overview
Manage submodules with scripts for add/remove/update/sync/status. Integrate select scripts as pre-commit hooks to automate checks. Run manually or via hooks. Use `chmod +x script.sh` for executability.

### Scripts

1. **add_submodule.sh**  
   Adds and initializes submodule.  
   Usage: `./add_submodule.sh <repo-url> <path>`  
   Call: Manually when adding new submodule.  
   Not for pre-commit (addition isn't a commit action).

2. **remove_submodule.sh**  
   Cleanly removes submodule.  
   Usage: `./remove_submodule.sh <path/to/submodule>`  
   Call: Manually when removing.  
   Not for pre-commit.

3. **update_submodules.sh**  
   Updates all submodules to latest.  
   Usage: `./update_submodules.sh`  
   Call: Manually after pull/clone, or in post-merge hook.  
   As pre-commit hook: Add to `.pre-commit-config.yaml` under `local` repo:  
   ```yaml
   - id: update-submodules
     name: Update submodules
     entry: ./update_submodules.sh
     language: script
     pass_filenames: false
   ```  
   (Runs on commit; adjust if auto-update desired.)

4. **sync_branches.sh**  
   Syncs submodules to specific branch.  
   Usage: `./sync_branches.sh [branch]` (default: main)  
   Call: Manually for branch alignment.  
   As pre-commit: Similar to above, but only if branch sync needed pre-commit.

5. **check_submodules.sh**  
   Checks status/dirty changes.  
   Usage: `./check_submodules.sh`  
   Call: Manually for inspection.  
   As pre-commit: Ideal for validation. Add:  
   ```yaml
   - id: check-submodules
     name: Check submodule status
     entry: ./check_submodules.sh
     language: script
     pass_filenames: false
   ```  
   (Blocks commit if issues.)

6. **setup.sh**  
   Initializes/updates submodules.  
   Usage: `./setup.sh`  
   Call: Post-clone/pull.  
   Not for pre-commit; use in onboarding/CI.

### Integration
- Install pre-commit: `pip install pre-commit` or `pixi add pre-commit`.  
- Add hooks to `.pre-commit-config.yaml` under `repos: - repo: local`.  
- Run `pre-commit install` to enable.  
- Hooks (e.g., check/update) run auto on `git commit`. Skip: `git commit --no-verify`.  
- For post-merge auto: Add to `.git/hooks/post-merge`: `./update_submodules.sh`.

### Best Practices
- Place scripts in root dir.  
- Doc in README: Include clone instructions, script usages.  
- Test: `pre-commit run --all-files`.