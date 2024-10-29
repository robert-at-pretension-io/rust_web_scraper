# LangChain Documentation

## Contents

- [Integrations](https://python.langchain.com/integrations/providers/)
- [API Reference](https://python.langchain.com/api_reference/)
- [Contributing](https://python.langchain.com/docs/contributing/)
- [People](https://python.langchain.com/docs/people/)
- [Error Reference](https://python.langchain.com/docs/troubleshooting/errors/)
- [LangSmith](https://docs.smith.langchain.com)
- [LangGraph](https://langchain-ai.github.io/langgraph/)
- [LangChain Hub](https://smith.langchain.com/hub)
- [LangChain JS/TS](https://js.langchain.com)

## Versions

- [v0.3](https://python.langchain.com/v0.3/docs/introduction/)
- [v0.2](https://python.langchain.com/v0.2/docs/introduction/)
- [v0.1](https://python.langchain.com/v0.1/docs/get_started/introduction)

---

## How-to Guides

This section contains answers to common "How do I...?" questions. These guides are **goal-oriented** and **concrete**, designed to help you complete specific tasks. For conceptual explanations, see the [Conceptual Guide](https://python.langchain.com/docs/concepts/). For end-to-end walkthroughs, see [Tutorials](https://python.langchain.com/docs/tutorials/). For comprehensive descriptions of every class and function, refer to the [API Reference](https://python.langchain.com/api_reference/).

### Installation

- [How to: install LangChain packages](https://python.langchain.com/docs/how_to/installation/)
- [How to: use LangChain with different Pydantic versions](https://python.langchain.com/docs/how_to/pydantic_compatibility/)

### Key Features

Core functionalities include:

- [How to: return structured data from a model](https://python.langchain.com/docs/how_to/structured_output/)
- [How to: use a model to call tools](https://python.langchain.com/docs/how_to/tool_calling/)
- [How to: stream runnables](https://python.langchain.com/docs/how_to/streaming/)
- [How to: debug your LLM apps](https://python.langchain.com/docs/how_to/debugging/)

### LangChain Expression Language (LCEL)

With [LangChain Expression Language](https://python.langchain.com/docs/concepts/lcel/) you can create arbitrary custom chains based on the [Runnable](https://python.langchain.com/api_reference/core/runnables/langchain_core.runnables.base.Runnable.html) protocol.

- **[LCEL Cheatsheet](https://python.langchain.com/docs/how_to/lcel_cheatsheet/)**: Quick overview of main primitives.
- **[Migration Guide](https://python.langchain.com/docs/versions/migrating_chains/)**: For transitioning from legacy chain abstractions to LCEL.

#### Key LCEL Guides

- [How to: chain runnables](https://python.langchain.com/docs/how_to/sequence/)
- [How to: stream runnables](https://python.langchain.com/docs/how_to/streaming/)
- [How to: configure runnable behavior at runtime](https://python.langchain.com/docs/how_to/configure/)
- [How to: add message history (memory) to a chain](https://python.langchain.com/docs/how_to/message_history/)

### Components

#### Prompt Templates

Responsible for formatting user input for language models.

- [How to: use few shot examples](https://python.langchain.com/docs/how_to/few_shot_examples/)
- [How to: partially format prompt templates](https://python.langchain.com/docs/how_to/prompts_partial/)

#### Example Selectors

Select correct examples for prompts.

- [How to: select examples by length](https://python.langchain.com/docs/how_to/example_selectors_length_based/)
- [How to: use example selectors](https://python.langchain.com/docs/how_to/example_selectors/)

#### Chat Models

Newer forms of models for conversational interfaces.

- [How to: do function/tool calling](https://python.langchain.com/docs/how_to/tool_calling/)
- [How to: cache model responses](https://python.langchain.com/docs/how_to/chat_model_caching/)

#### LLMs

Older forms of language models.

- [How to: create a custom LLM class](https://python.langchain.com/docs/how_to/custom_llm/)
- [How to: cache model responses](https://python.langchain.com/docs/how_to/llm_caching/)

#### Output Parsers

Parse LLM outputs into structured formats.

- [How to: parse JSON output](https://python.langchain.com/docs/how_to/output_parser_json/)
- [How to: write a custom output parser class](https://python.langchain.com/docs/how_to/output_parser_custom/)

#### Document Loaders

Load documents from various sources.

- [How to: load PDF](https://python.langchain.com/docs/how_to/document_loader_pdf/)
- [How to: load HTML](https://python.langchain.com/docs/how_to/document_loader_html/)

#### Text Splitters

Split documents into manageable chunks.

- [How to: split by headers](https://python.langchain.com/docs/how_to/markdown_header_metadata_splitter/)
- [How to: recursively split text](https://python.langchain.com/docs/how_to/recursive_text_splitter/)

#### Embedding Models

Create numerical representations of text.

- [How to: embed text data](https://python.langchain.com/docs/how_to/embed_text/)

#### Vector Stores

Efficiently store and retrieve embeddings.

- [How to: use a vector store to retrieve data](https://python.langchain.com/docs/how_to/vectorstores/)

#### Retrievers

Return relevant documents based on queries.

- [How to: use hybrid vector and keyword retrieval](https://python.langchain.com/docs/how_to/hybrid/)
- [How to: reorder retrieved results](https://python.langchain.com/docs/how_to/long_context_reorder/)

#### Tools

Describe tools with implementations to combine with language models.

- [How to: create tools](https://python.langchain.com/docs/how_to/custom_tools/)
- [How to: use built-in tools and toolkits](https://python.langchain.com/docs/how_to/tools_builtin/)

#### Agents

For in-depth guides, refer to [LangGraph documentation](https://langchain-ai.github.io/langgraph/).

- [How to: use legacy LangChain Agents](https://python.langchain.com/docs/how_to/agent_executor/)

#### Callbacks

Hooks into various execution stages of your application.

- [How to: pass in callbacks at runtime](https://python.langchain.com/docs/how_to/callbacks_runtime/)

#### Custom

Extend components to create custom versions.

- [How to: create a custom chat model class](https://python.langchain.com/docs/how_to/custom_chat_model/)

#### Serialization

- [How to: save and load LangChain objects](https://python.langchain.com/docs/how_to/serialization/)

## Use Cases

### Q&A with RAG

Connect LLMs to external data sources.

- [How to: add chat history](https://python.langchain.com/docs/how_to/qa_chat_history_how_to/)

### Extraction

Extract structured information from unstructured text.

- [How to: handle long text](https://python.langchain.com/docs/how_to/extraction_long_text/)

### Chatbots

Build conversational LLM applications.

- [How to: manage memory](https://python.langchain.com/docs/how_to/chatbots_memory/)

### Query Analysis

Generate queries for retrievers using LLMs.

- [How to: handle cases where no queries are generated](https://python.langchain.com/docs/how_to/query_no_queries/)

### Summarization

Distill information from text.

- [How to: summarize text in a single LLM call](https://python.langchain.com/docs/how_to/summarize_stuff/)

## Related Resources

- [LangGraph](https://langchain-ai.github.io/langgraph/)
- [LangSmith](https://docs.smith.langchain.com/)

---

For further assistance, check community discussions on [Twitter](https://twitter.com/LangChainAI) or see the [GitHub repository](https://github.com/langchain-ai/langchain).