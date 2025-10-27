# Component Based Programming, Base Template
***This base component contains verything that is not language specific.***
<br>
<br>
## Thoughts

- There must be a common project structure for the various language concerns.
```
- {{component_name}}
    - docs
        - templates (for mkdocs)
    - src (shown for python, but should be generalized across languages)
        - {{component_name_slug}}
            - resolvers
                - __init__.py
            - types (import the resolvers as needed. should be organized for implied interface across variations)
                - __init__.py
            - schemas
                - __init__.py
            - configs
                - __init__.py
            - utils
                - __init__.py
            - __init__.py
    - tests
    - README.md
    - LICENSE.md
    - CONTRIBUTIONS.md
    - CHANGES.md
    - pixi.toml
    - pixi.lock
    - .gitignore
    - .envrc
    - mkdocks.yaml
    - cbp-component.yaml
```