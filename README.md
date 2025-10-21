# Component Based Programming
An alternative to conventional programming.
<br>
<br>
<br>
<br>
## Table of Contents
[New Architecture](#new-architecture)
- [Python CLI Interface](#python-cli-interface)
- [Components](#components)
<br>
<br>
<br>
<br>


## New Architecture

- `cbp` is the cli interface to launchers and everything meta.
- The gateway will be in rust and most efficient.

    - All language servers are launched from here dynamically.
    - Gateway will merely be responsible for:

        - Routing map ~ Graphql
        - Dynamic CBP language server/router launches and maintenance
        - Dynamic CBP `Task` routing via the language servers
        - Async, tokio event loop w/ zmq, etc.


### Python CLI Interface

- Launcher
    - Servers/Daemons
        - Gateway: The rust, reverse proxy server ..the interface to the CBP world.
            - Language: Async language servers for each language (Gateway launches these ondemand)
                - Python
                - JS
                - Rust ? (can just use gateway?)
- All meta and processing interface to CBP systems

### Components

- Modular packages
    - `Task` Code (or link to where it is hosted)
    - CLI interface code (plugin type)
    - Web Component code
        - Perfect for CBP
            - each web-component is aligned with the business logic
                - Input types
                - Output types

    - Types are the contract (as is the graphql interface)
        - Inputs/Outputs.. thats it.
        - Pydantic/Graphql/etc 
            - All dynamically, automatically translated and generated/launched.


