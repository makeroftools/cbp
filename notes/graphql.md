# Notes on GraphQL and Strawberry, etc

## Concepts and Thoughts

* Using sqlmodel types.. they are pydantic types too.
* Strawberry has a conversion generator, but it is terrible.

### Types

* Custom type definitions
* Query, Mutation, Subscription
* Resolvers
* Input Variables
* Output

## My Goals

* Everything is dynamically generated
  * Task Declarations are kept in the gateway db.. the gateway is responsible for the "SHELL"
  * The gateway is the CBP controller, router, scheduler
  * Every Task is executed by the gateway
    * Determines the execution flow by examining its `children` field
  * Where does the Task end and the Component begin???
    * The component is  a wrapper around the task.. what does that mean exactly? What does a component contain exactly?
