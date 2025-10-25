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
- [Component Templates](#component-templates)
- [The Gateway](#the-gateway)
<br>
<br>
<br>
<br>


## New Architecture

- `cbp` is the cli interface to launchers and everything meta.
- The gateway will be in rust and most efficient.

    - All language servers are launched from here dynamically.
    - Gateway will merely be responsible for:

        - Routing map ~ Graphql Federation Router
        - Dynamic CBP language server/router launches and maintenance 
            - via various loosely-coupled services
        - Dynamic CBP `Task` routing via the language servers
        - Async, tokio event loop w/ zmq, grpc, tcp, rest, db, etc.

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
            - ..handles file sockets, zero-copy, etc.
            - ..or better.. like maybe direct event-loop integration.. I think zmq does this..?
    - Each is hosted in some (language) execution environment
        - As part of an executing async event loop
            - Offloads to process/thread pools when relatively intense processing is needed.

### Component Templates

- `Copier` component project templates.
- Provides all the project boilerplate items
- Interactive dynamic configurations
- Each component is generated from a component template.
    - Language orientated.
        - Language Server
            - Async event loop at its core.
            - Api server using graphql
                - Awesome, hierarchical formalization and object retreival.
            - Efficiency orientated.
        - Languages
            - Python
            - Rust
            - Mojo
            - NodeJs
            - Bash
            - C
            - C++
                - std
                - Qt
            - Go
            - etc.

    - Copier project templates
        - Pixi package manager
        - rattler-build ??
        - all source-repos go into the `src/` directory.
        - all executable binaries go into `bin/` or `target` directories.
        - all compiled and needed C/C++ libraries and headers go into `lib/` and `include/` directories.
        - Dynamically generated orientated towards the component's runtime language.
            - Copier `help/answers` formatted for this decision and immediate generation and build.


### The Gateway

- Rust Async api server using graphql
    - Relays messages from the intertubes to the cbp "system of systems".
    - Is a graphql "Apollo Federation" compliant server.
    - async-graphql + poem == "Success"
    - Crates
        - zmq
        - tokio
        - poem
        - msgpack
    - 

### Language Servers

- All are "Apollo Federation" compliant.

- **Python** 
    - FastAPI and Strawberry... mmm mm good!

- **Typescript**
    - Hive, etc.

- **Rust**
    - async-graphql and Poem.

- **Go**
    - ?

- **C/C++**
    - Probably via a rust service.



### Graphical User Interface

- Every component will utilize a web component (www standard).
    - Web components are beautiful, just like cbp components ;-).
- The "shell" or "dashboard" or "portal" will be the landing.
    - It is the house of the "system of systems" that is cbp.


### CLI interface

- cbp will contain a client to the graphql gateway.
- Each component will contain a snippet/plugin for the cli.



### Testing
#### Testing Setup for Multi-Language Async GraphQL Gateway

**Layered Approach:**
- **Unit Tests:** Isolate components in each language (e.g., Rust's `cargo test`, Python's pytest); mock messaging/GraphQL calls.
- **Contract Tests:** Use Pact for GraphQL schemas (consumer: define queries/mutations; provider: verify responses) and async messages (consumer: expect payloads; provider: generate matching events). Supports Rust via pact-rs library.
- **Integration Tests:** Virtualize services (e.g., Parasoft tools) to simulate language servers/messaging queues; test gateway routing.
- **E2E Tests:** Run sparingly on full system; focus on critical paths.

**Best Practice:** Consumer-driven contracts ensure cross-language compatibility without full deployments; automate in CI/CD.