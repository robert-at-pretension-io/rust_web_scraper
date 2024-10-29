```markdown
# Introduction

**LangChain** is a framework for developing applications powered by large language models (LLMs).

LangChain simplifies every stage of the LLM application lifecycle:

- **Development**: Build your applications using LangChain's open-source [building blocks](#), [components](#), and [third-party integrations](#). Use [LangGraph](#) to build stateful agents with first-class streaming and human-in-the-loop support.
- **Productionization**: Use [LangSmith](#) to inspect, monitor, and evaluate your chains, so that you can continuously optimize and deploy with confidence.
- **Deployment**: Turn your LangGraph applications into production-ready APIs and Assistants with [LangGraph Cloud](#).

![Diagram outlining the hierarchical organization of the LangChain framework](#)

Concretely, the framework consists of the following open-source libraries:

- **`langchain-core`**: Base abstractions and LangChain Expression Language.
- **`langchain-community`**: Third-party integrations.
  - Partner packages (e.g. **`langchain-openai`**, **`langchain-anthropic`**, etc.): Some integrations have been further split into their own lightweight packages that only depend on **`langchain-core`**.
- **`langchain`**: Chains, agents, and retrieval strategies that make up an application's cognitive architecture.
- **[LangGraph](#)**: Build robust and stateful multi-actor applications with LLMs by modeling steps as edges and nodes in a graph. Integrates smoothly with LangChain, but can be used without it.
- **[LangServe](#)**: Deploy LangChain chains as REST APIs.
- **[LangSmith](#)**: A developer platform that lets you debug, test, evaluate, and monitor LLM applications.

These docs focus on the Python LangChain library. [Head here](#) for docs on the JavaScript LangChain library.

## Tutorials

If you're looking to build something specific or are more of a hands-on learner, check out our [tutorials section](#). This is the best place to get started.

### Suggested Tutorials

- [Build a Simple LLM Application](#)
- [Build a Chatbot](#)
- [Build an Agent](#)
- [Introduction to LangGraph](#)

Explore the full list of LangChain tutorials [here](#), and check out other [LangGraph tutorials here](#). To learn more about LangGraph, check out our first LangChain Academy course, *Introduction to LangGraph*, available [here](#).

## How-to Guides

[Here](#) you’ll find short answers to “How do I….?” types of questions. These how-to guides don’t cover topics in depth – you’ll find that material in the [Tutorials](#) and the [API Reference](#). However, these guides will help you quickly accomplish common tasks.

Check out [LangGraph-specific how-tos here](#).

## Conceptual Guide

Introductions to all the key parts of LangChain you’ll need to know! [Here](#) you'll find high-level explanations of all LangChain concepts.

For a deeper dive into LangGraph concepts, check out [this page](#).

## API Reference

Head to the reference section for full documentation of all classes and methods in the LangChain Python packages.

## Ecosystem

### [🦜🛠️ LangSmith](#)

Trace and evaluate your language model applications and intelligent agents to help you move from prototype to production.

### [🦜🕸️ LangGraph](#)

Build stateful, multi-actor applications with LLMs. Integrates smoothly with LangChain but can be used without it.

## Additional Resources

### Versions

See what changed in v0.3, learn how to migrate legacy code, read up on our versioning policies, and more.

### Security

Read up on [security](#) best practices to make sure you're developing safely with LangChain.

### Integrations

LangChain is part of a rich ecosystem of tools that integrate with our framework and build on top of it. Check out our growing list of [integrations](#).

### Contributing

Check out the developer's guide for guidelines on contributing and help getting your dev environment set up.

---

#### Was this page helpful?

- [Next: Tutorials](#)
```