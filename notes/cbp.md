# Component Based Programming CLI

## Ranom thoughts atm

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