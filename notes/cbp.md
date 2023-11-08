# Component Based Programming CLI

* A component can be a task or type (verb or noun)
* components are either atomic or composite
* composites are networks of sub-components
* task components
  * a callable
    * async
    * process
  * ui
    * properties of output and task meta
  * dockerfile

---

---

## Random thoughts atm

* Building and updating publicly
* Proper OSS project on github
* Use github actions too.. maybe.. could just mirror private repo on private host
* This is going to initially be my playground and where I'll start to build the infrastructure,

  but only according to my immediate needs.. iow FOR what I need it for.
* Coffee is ready!

---

* Hmm, lets try something

```

[This is a node]-----------[This is another node]

                    |

                    |

            [Here is a child node]

```

* OK.. need an AI to translate graph designs in Markdown and elsewhere
* Jupyter orientated AI agent

---

## Thoughts on Infrastructure

* It all begins with the USER (aka: The Portal)
* If WEB
* Interface is with a web-server (most likely and defaultly remotely hosted in the cloud)
* If CLI
* its in it.. can:
* launch a task (component of compoents)
* This entails launching any component language servers needed

## Thoughts on the UI

* This is "The Portal"
* The Imersive User Interface is f**** awesome!
* Until then..
* CLI will have the capability
* all I need atm

## What is a Component

It..

* executes a task to serve a goal that is used primarily by the portal
* is anyone of the languages that zeromq is ported to.
* can be generated, provisioned and activated within a dynamically generated network (another component.. lol)
* its execution begins at the portal or an "automation scheduler" component
* has a generator that brings ANY incarnation of a component to life
* ..the generator uses the component's "dna".. (hmm need to incorpoarate Holochain)
* A yaml file.. declarative!
* So... lets define it in the form of a project cookiecutter/generator
* use cookiecutter but only from python
* make the root directory a python poetry project and call cookiecutter from it.
* do this in the cbp CLI
* one cookiecutter for each language
* spins up a task server per language.
* Each component comes in only ONE language
* higher level languages can be converted to faster implementations.
* AI
* Human curated when needed

## What defines a component?

* The component provides the necessary "pieces" that make up a component
  * The task (is a description and meta to perform the execution)
  * The task in turn is a list of "child" tasks.
  * A Graphql Description
    * The signature of the output
    * The signature of the input
    * The client code

# Infrastructure and Component Distinction or not

* Gateway
* Language Task Server (Python ~ FastAPI)
* How big (monolithic) can a component be?
* What is a Component again? ..a Task?
* 

### Type

* Component:
  * Task:
    * Input (graphql definition)
    * Execution Block (code, can be empty and simply passed to children sequence)
    * Output (graphql definition)
    * Children Tasks
    * Meta:
      * Profile
      * Description
      * Name
  * Host:
    * Async Server, is the controlling entity for all execution methods.. the buck starts here
    * Process, spawned by the async-server
  * Communication: (zmq, graphql, http)
    * Sockets are dynamically added as needed
      * I need to develop a
  * UI (a suplied web component used a default and is usually overridden )







**/**# Model-based ML Towards Causal Reasoning in an AI Scientist by Yoshua Bengio (Turing Award Winner) - YouTube.md
