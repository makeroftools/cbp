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
        - Dynamically generated, orientated towards the component's runtime language.
            - Copier `help/answers` formatted for this decision and immediate generation and build.


### The Gateway

- Hive Router
    - Only concern is the super-graph and its operation from the registered subgraphs

### Language Servers

- All are "Apollo Federation" compliant.

- **Python** 
    - FastAPI and Strawberry... mmm mm good!

- **Typescript**
    - Yoga, etc.

- **Rust**
    - async-graphql and Poem.

- **Go**
    - ?

- **C/C++**
    - Probably via a rust service.

- **Bash**
    - From the rust service.
    - ..or from the python service.. convenient.



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


## Thoughts 25.11.06
    Everything gets generated (with cached/registered components available for quick access)

### MetaSyntax Programming

- Not even grammar standards are standard.. mfs
- Build a lanugage hierarchy
- And a grammar language hierarchy
- Do this in a graphql schema consideration

#### "Glue"

Like python is to other programming languages, glue is the consideration (the portal) to metasyntax languages too.

- I should also orientate towards "Context-Sensitive Flow-based programming"
    - I should translate all bnfs to CBP metasytax (I don't have one..??)
    - CBP metasyntax is a superset of all other metasyntax languages




Language is a formalization
An agreed upon communication protocol between agents


tasks ARE types 
    - They output a type
    - So, the the depenent and the independent variables ARE types.. ergo, a task is a type with an outcome that is the type.
    - so, 


#### Phonemes and Meanings


#### Key Linguistic Theories:

- **Structuralism** (Saussure): 
    Language as a system of signs; 
    focuses on relationships between elements like phonemes and meanings.

- **Generative Grammar** (Chomsky): 
    Innate universal grammar; 
    rules generate infinite sentences from finite structures.

- **Functionalism**: 
    Language shaped by communicative needs; 
    emphasizes usage in social contexts.

- **Cognitive Linguistics**: 
    Language reflects human cognition; 
    concepts like metaphor and embodiment central.

- **Sociolinguistics**: 
    Studies language variation by society, culture, and power dynamics.



### Tree-sitter Notes

- Basic Objects
    - Language
    - Syntax Tree


### Language Hierarchy

**Prompt:**
    Given the term "Language", give me it's constituent parts/considerations. I need an expert's report. I would want to try to categorize all languages, natural, formal or otherwise.

**Response**
### Expert Report on "Language": Constituent Parts and Categorizations

#### Constituent Parts/Considerations
Languages comprise:
- **Syntax**: Rules for structuring symbols/words into valid sequences.
- **Semantics**: Meaning assignment to symbols/structures.
- **Phonology/Morphology** (natural langs): Sound systems and word formation.
- **Pragmatics**: Contextual use and interpretation.
- **Lexicon**: Vocabulary set.
- **Grammar**: Overall rule system.
Key considerations: Expressivity, ambiguity, evolution (natural) vs. design (formal), universality vs. specificity.

#### Categorization of All Languages
1. **Natural Languages**: Evolved organically in human societies.
   - Spoken: English, Mandarin (phonetic-based).
   - Written: Extensions of spoken (alphabetic, logographic).
   - Sign: ASL (visual-gestural).
   - ~7,000 extant; families like Indo-European, Sino-Tibetan.

2. **Constructed Languages (Conlangs)**: Human-designed.
   - Auxiliary: Esperanto (international communication).
   - Artistic/Fictional: Klingon (entertainment).
   - Engineered: Lojban (logical precision).

3. **Formal Languages**: Designed for precise, unambiguous computation/mathematics.
   - Programming: Python, C++ (machine-executable).
   - Logical/Mathematical: Predicate logic, set theory notations.
   - Query: SQL (data manipulation).

4. **Other**: 
   - Animal communication: Bee dances, whale songs (proto-languages, limited syntax/semantics).
   - Non-verbal: Body language, emojis (supplementary systems).


Language  
├── Components  
│   ├── Syntax (structure rules)  
│   ├── Semantics (meaning)  
│   ├── Phonology/Morphology (sounds/words; natural only)  
│   ├── Pragmatics (context)  
│   ├── Lexicon (vocabulary)  
│   └── Grammar (overall rules)  
└── Categories  
    ├── Natural (evolved; ~7k families)  
    │   ├── Spoken (e.g., English)  
    │   ├── Written (alphabetic/logographic)  
    │   └── Sign (e.g., ASL)  
    ├── Constructed (designed)  
    │   ├── Auxiliary (e.g., Esperanto)  
    │   ├── Artistic (e.g., Klingon)  
    │   └── Engineered (e.g., Lojban)  
    ├── Formal (precise/computational)  
    │   ├── Programming (e.g., Python)  
    │   ├── Logical (e.g., predicate logic)  
    │   └── Query (e.g., SQL)  
    └── Other  
        ├── Animal (e.g., bee dances)  
        └── Non-verbal (e.g., emojis)


### Head Out of Ass.. moment.

My Goal: 
    Easy access to exchanges in a lower level language that can be glued with python

The Problem: 
    Everything available sucks!

The Constraint:
    - Functionality is available sparsely amongst open source projects and programming languages.
    - Some programming languages and methodologies are not acceptable.

The Ideal:
    - What I'm trying to steer away from right now.
    - A magic wand that produces what I want, when I want it.


### CSG Resources

[Stackoverflow: Is there a parsing algorithm for languages generated by context-sensitive grammars?](https://stackoverflow.com/questions/76810761/is-there-a-parsing-algorithm-for-languages-generated-by-context-sensitive-gramma)

[https://en.wikipedia.org/wiki/Linear_bounded_automaton](https://en.wikipedia.org/wiki/Linear_bounded_automaton)

[maandree/libparser](https://github.com/maandree/libparser)

[w-wieczorek/ConsoleCSI](https://github.com/w-wieczorek/ConsoleCSI)

[google/compynator](https://github.com/google/compynator)

[GeeksForGeeks: Context-sensitive Grammar (CSG) and Language (CSL)](https://www.geeksforgeeks.org/theory-of-computation/context-sensitive-grammar-csg-and-language-csl/)

[Tomassetti: Guide to Parsing Algorithms](https://tomassetti.me/guide-parsing-algorithms-terminology/)

[Toolbox for Context-Sensitive Grammar Induction](https://proceedings.mlr.press/v153/wieczorek21a/wieczorek21a.pdf)

[Algorithms for Scalable Natural Language Processing (TAG)](https://etd.ohiolink.edu/acprod/odb_etd/ws/send_file/send?accession=case1465469914&disposition=attachment)


***Definition of Parsing***
    - The analysis of an input, to organize the data, according to the rule of a grammar.
    - To find the underlying structure of the data we are given.

***Definition of a Grammar***
    - A formal grammar is a set of rules that syntactically describes a language.
    - In formal language theory, a grammar is a quadruple (V, Σ, R, S) where 
        - V is variables, 
        - Σ is terminals, 
        - R is production rules, and 
        - S is the start symbol.


### Notes on Parsing, generally

***Note: Raw representation is usually text, can be binary or other more exotics like sound, characteristics, other.***
- Is the inverse of templating, some what.
    - parsing: identifying structure to extract the data
        - Detemine the model from the raw representation.
        - Input: grammar (rules) and raw representation
        - Output: Identification of structure, data model
    - templating: Have structure, fill in with data.
        - Combine the data w/ the model from the raw_representation
        - Input: template (structure), data model
        - Output: raw representation
- Production Rules:
    - Technically, it defines a transformation between the nonterminal, on the left, and the set of nonterminals and terminals, on the right.


Build this lexer/parser/generator also in CBP format.. network of networks



Input --> Lexical analysis --> Token sequence (+grammar definition) --> Parser --> Parse Tree or an AST (a form of IR) --> IR? --> 



    Input
        |
        |
        /\
       /  \
      /    \
    Branch1 Branch2


Parse the input.. stitch the tree.. If not validated.. alternate action (new tree if there is one)