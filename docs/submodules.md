[how do incorporate web based project into pixi project manager?](https://grok.com/share/bGVnYWN5LWNvcHk_366c2878-215f-4f00-a70b-516dfa3eab4b)


## To make git submodules painless:

1. **Clone with submodules**: Always use  
   `git clone --recurse-submodules https://github.com/your/repo.git`

2. **Create setup script** (`setup.sh` in root):  
   ```bash
   #!/bin/bash
   echo "Initializing and updating submodules..."
   git submodule update --init --recursive
   echo "Done. Ready to work."
   ```
   Make executable: `chmod +x setup.sh`

3. **Add to README.md** (top section):  
   ```
   ## Setup
   After cloning:
   git clone --recurse-submodules <url>
   OR
   ./setup.sh
   ```

4. **Pre-commit hook** (install pre-commit):  
   - `pip install pre-commit` (or via pixi)  
   - Create `.pre-commit-config.yaml` in root:  
     ```yaml
     repos:
       - repo: https://github.com/jeremyephron/pre-commit-git-submodule
         rev: v1.0.0
         hooks:
           - id: submodule-status
     ```
   - Run `pre-commit install`  
   This blocks commits if submodules are dirty or not updated.

5. **Post-pull automation**: Add to `.git/hooks/post-merge` (make executable):  
   ```bash
   #!/bin/sh
   git submodule update --init --recursive
   ```

***These steps hide 95% of submodule pain for team/onboarding.***

## To cleanly remove a Git submodule:

1. Deinitialize:  
   `git submodule deinit -f -- path/to/submodule`

2. Remove from index:  
   `rm -rf .git/modules/path/to/submodule`  
   `git rm -f path/to/submodule`

3. Remove from .git/config (if needed):  
   Manually edit `.git/config` and delete the submodule section.

4. Commit changes:  
   `git commit -m "Remove submodule path/to/submodule"`

5. Delete leftover folder (if still exists):  
   `rm -rf path/to/submodule`

Done. No traces remain.