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

- Everything is a component
- Modular packages
    - `CBP_Task` Code (or link to where it is hosted)
    - CLI interface (typer) code (plugin type)
    - Web Component code
        - Perfect for CBP
            - each web-component is aligned with the business logic
                - Input types
                - Output types
    - Use `rattler-build` to build and package components
    - Types are the contract (as is the graphql interface)
        - Inputs/Outputs.. thats it.
        - Pydantic/Graphql/etc 
            - All dynamically, automatically translated and generated/launched.
- Dynamic hosting and execution environments
    - Each component is in an async event system
        - zmq::poll is used as the input/output controller for each
    - Each is hosted in some (language) execution environment
        - As part of an executing async event loop
            - Offloads to process/thread pools when intense processing needed.


