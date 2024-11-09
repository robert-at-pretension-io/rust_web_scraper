

```markdown
# Realtime API

## Client Events

### session.update

- **Purpose**: Update session’s default configuration.
- **Fields**:
  - `event_id`: *string* (Optional) - Identifier for the event.
  - `type`: *string* - Must be "session.update".
  - `session`: *object* - Realtime session object configuration.
- **Example**:
  ```json
  {
      "event_id": "event_123",
      "type": "session.update",
      "session": {
          "modalities": ["text", "audio"],
          "instructions": "Your knowledge cutoff is 2023-10. You are a helpful assistant.",
          "voice": "alloy",
          ...
      }
  }
  ```

### input_audio_buffer.append

- **Purpose**: Append audio bytes to the input audio buffer.
- **Fields**:
  - `event_id`: *string* (Optional) - Identifier for the event.
  - `type`: *string* - Must be "input_audio_buffer.append".
  - `audio`: *string* - Base64-encoded audio bytes.
- **Example**:
  ```json
  {
      "event_id": "event_456",
      "type": "input_audio_buffer.append",
      "audio": "Base64EncodedAudioData"
  }
  ```

### input_audio_buffer.commit

- **Purpose**: Commit user input audio buffer to create a new user message.
- **Fields**:
  - `event_id`: *string* (Optional) - Identifier for the event.
  - `type`: *string* - Must be "input_audio_buffer.commit".
- **Example**:
  ```json
  {
      "event_id": "event_789",
      "type": "input_audio_buffer.commit"
  }
  ```

### conversation.item.create

- **Purpose**: Add a new item to the conversation.
- **Fields**:
  - `event_id`: *string* (Optional) - Identifier for the event.
  - `type`: *string* - Must be "conversation.item.create".
  - `previous_item_id`: *string* - ID of the preceding item.
  - `item`: *object* - The item to add.
- **Example**:
  ```json
  {
      "event_id": "event_345",
      "type": "conversation.item.create",
      "previous_item_id": null,
      "item": {
          "id": "msg_001",
          ...
      }
  }
  ```

### response.create

- **Purpose**: Trigger model inference to create a Response.
- **Fields**:
  - `event_id`: *string* (Optional) - Identifier for the event.
  - `type`: *string* - Must be "response.create".
  - `response`: *object* - The response resource.
- **Example**:
  ```json
  {
      "event_id": "event_234",
      "type": "response.create",
      "response": {
          ...
      }
  }
  ```

## Server Events

### error

- **Purpose**: Indicate an error occurred.
- **Fields**:
  - `event_id`: *string* - Unique ID of the server event.
  - `type`: *string* - Must be "error".
  - `error`: *object* - Details of the error.
- **Example**:
  ```json
  {
      "event_id": "event_890",
      "type": "error",
      "error": {
          "type": "invalid_request_error",
          ...
      }
  }
  ```

### session.created

- **Purpose**: Notify that a session is created.
- **Fields**:
  - `event_id`: *string* - Unique ID of the server event.
  - `type`: *string* - Must be "session.created".
  - `session`: *object* - Realtime session object configuration.
- **Example**:
  ```json
  {
      "event_id": "event_1234",
      "type": "session.created",
      ...
  }
  ```

### conversation.item.created

- **Purpose**: Indicate a new conversation item is created.
- **Fields**:
  - `event_id`: *string* - Unique ID of the server event.
  - `type`: *string* - Must be "conversation.item.created".
  - `previous_item_id`: *string* - ID of the preceding item.
  - `item`: *object* - The item to add.
- **Example**:
  ```json
  {
      "event_id": "event_1920",
      "type": "conversation.item.created",
      ...
  }
  ```

### conversation.item.input_audio_transcription.completed

- **Purpose**: Signal completion of audio transcription for user audio.
- **Fields**:
  - `event_id`: *string* - Unique ID of the server event.
  - `type`: *string* - Must be "conversation.item.input_audio_transcription.completed".
  - `item_id`: *string* - ID of the user message item.
  - `content_index`: *integer* - Index of the content part with the audio.
  - `transcript`: *string* - The transcribed text.
- **Example**:
  ```json
  {
      "event_id": "event_2122",
      "type": "conversation.item.input_audio_transcription.completed",
      "item_id": "msg_003",
      ...
  }
  ```

## Conclusion

The Realtime API allows for seamless interaction and communication in real time, providing functionalities to update sessions, manage audio buffers, and handle conversations. Detailed error and event responses help in debugging and ensuring smooth interaction with the system.
```

# Realtime API Events

## Events Overview

The realtime API involves several events that are triggered under different scenarios during server-client interactions. Below are the detailed specifications of each event type.

### `input_audio_buffer.committed`

An event fired when a user message item is created by committing the input audio buffer.

```json
{
    "event_id": "event_1121",
    "type": "input_audio_buffer.committed",
    "previous_item_id": "msg_001",
    "item_id": "msg_002"
}
```

- **Properties:**
  - `event_id`: String, a unique identifier for the event.
  - `type`: String, must be `input_audio_buffer.committed`.
  - `previous_item_id`: String, ID of the preceding item.
  - `item_id`: String, ID of the user message item to be created.

### `input_audio_buffer.cleared`

Indicates the clearing of the input audio buffer.

```json
{
    "event_id": "event_1314",
    "type": "input_audio_buffer.cleared"
}
```

- **Properties:**
  - `event_id`: String, a unique identifier for the event.
  - `type`: String, must be `input_audio_buffer.cleared`.

### `input_audio_buffer.speech_started`

Triggered when the server detects the start of speech in the audio buffer.

```json
{
    "event_id": "event_1516",
    "type": "input_audio_buffer.speech_started",
    "audio_start_ms": 1000,
    "item_id": "msg_003"
}
```

- **Properties:**
  - `event_id`: String, a unique identifier for the event.
  - `type`: String, must be `input_audio_buffer.speech_started`.
  - `audio_start_ms`: Integer, milliseconds from the session start when speech was detected.
  - `item_id`: String, ID of the user message item to be created when speech stops.

### `input_audio_buffer.speech_stopped`

This event occurs when the server detects an end of speech.

```json
{
    "event_id": "event_1718",
    "type": "input_audio_buffer.speech_stopped",
    "audio_end_ms": 2000,
    "item_id": "msg_003"
}
```

- **Properties:**
  - `event_id`: String, a unique identifier for the event.
  - `type`: String, must be `input_audio_buffer.speech_stopped`.
  - `audio_end_ms`: Integer, milliseconds from the session start when speech stopped.
  - `item_id`: String, ID of the user message item created.

### `response.created`

Denotes the creation of a new Response object, initially in the `in_progress` state.

```json
{
    "event_id": "event_2930",
    "type": "response.created",
    "response": {
        "id": "resp_001",
        "object": "realtime.response",
        "status": "in_progress",
        "status_details": null,
        "output": [],
        "usage": null
    }
}
```

- **Properties:**
  - `event_id`: String, a unique identifier for the event.
  - `type`: String, must be `response.created`.
  - `response`: Object, details about the response.

### `response.done`

Signals the completion of a Response, including all output items but excluding raw audio data.

```json
{
    "event_id": "event_3132",
    "type": "response.done",
    "response": {
        "id": "resp_001",
        "object": "realtime.response",
        "status": "completed",
        "output": [
            {
                "id": "msg_006",
                "object": "realtime.item",
                "type": "message",
                "content": [
                    {
                        "type": "text",
                        "text": "Sure, how can I assist you today?"
                    }
                ]
            }
        ],
        "usage": {
            "total_tokens": 275,
            "input_tokens": 127,
            "output_tokens": 148
        }
    }
}
```

- **Properties:**
  - `event_id`: String, a unique identifier for the event.
  - `type`: String, must be `response.done`.
  - `response`: Object, detailed response information.

### `response.output_item.added`

This event indicates the addition of a new item during Response generation.

```json
{
    "event_id": "event_3334",
    "type": "response.output_item.added",
    "response_id": "resp_001",
    "output_index": 0,
    "item": {
        "id": "msg_007",
        "object": "realtime.item",
        "status": "in_progress",
        "role": "assistant",
        "content": []
    }
}
```

- **Properties:**
  - `event_id`: String, a unique identifier for the event.
  - `type`: String, must be `response.output_item.added`.
  - `response_id`: String, ID of the Response to which the item belongs.
  - `output_index`: Integer, index of the output item.
  - `item`: Object, details of the item.

### `response.output_item.done`

Indicates that an item has completed streaming, or the streaming has been interrupted.

```json
{
    "event_id": "event_3536",
    "type": "response.output_item.done",
    "response_id": "resp_001",
    "output_index": 0,
    "item": {
        "id": "msg_007",
        "object": "realtime.item",
        "status": "completed",
        "role": "assistant",
        "content": [
            {
                "type": "text",
                "text": "Sure, I can help with that."
            }
        ]
    }
}
```

- **Properties:**
  - `event_id`: String, a unique identifier for the event.
  - `type`: String, must be `response.output_item.done`.
  - `response_id`: String, ID of the Response.
  - `output_index`: Integer, index of the output item.
  - `item`: Object, containing the completed item.

... 

The above specifications provide detailed guidance for the server/client implementations of real-time API event handling and tracking. Each event is equipped with the required JSON structure, ensuring clear, consistent communications between the server and client applications.

# Real-Time API Overview

## Retrieve File from Message

### Request

- **Endpoint**: `GET https://api.openai.com/v1/threads/{thread_id}/messages/{message_id}/files/{file_id}`
- **Parameter**: 
  - `file_id` (string, required): The ID of the file being retrieved.

### Response

Returns the `message file` object.

**Example Request**
```bash
curl https://api.openai.com/v1/threads/thread_abc123/messages/msg_abc123/files/file-abc123 \
  -H "Authorization: Bearer $OPENAI_API_KEY" \
  -H "Content-Type: application/json" \
  -H "OpenAI-Beta: assistants=v1"
```

**Example Response**
```json
{
  "id": "file-abc123",
  "object": "thread.message.file",
  "created_at": 1699061776,
  "message_id": "msg_abc123"
}
```

## Modify Message

### Request

- **Endpoint**: `POST https://api.openai.com/v1/threads/{thread_id}/messages/{message_id}`
- **Path Parameters**:
  - `thread_id` (string, required): The ID of the thread.
  - `message_id` (string, required): The ID of the message to modify.
- **Request Body**:
  - `metadata` (map, optional): Key-value pairs for additional information.

### Response

Returns the modified `message` object.

**Example Request**
```bash
curl https://api.openai.com/v1/threads/thread_abc123/messages/msg_abc123 \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $OPENAI_API_KEY" \
  -H "OpenAI-Beta: assistants=v1" \
  -d '{
      "metadata": {
        "modified": "true",
        "user": "abc123"
      }
    }'
```

**Example Response**
```json
{
  "id": "msg_abc123",
  "object": "thread.message",
  "created_at": 1699017614,
  "thread_id": "thread_abc123",
  "role": "user",
  "content": [
    {
      "type": "text",
      "text": {
        "value": "How does AI work? Explain it in simple terms.",
        "annotations": []
      }
    }
  ],
  "file_ids": [],
  "assistant_id": null,
  "run_id": null,
  "metadata": {
    "modified": "true",
    "user": "abc123"
  }
}
```

## Message Object

Represents a message within a thread.

### Fields

- `id` (string): The identifier.
- `object` (string): The object type, always `thread.message`.
- `created_at` (integer): Unix timestamp for creation.
- `thread_id` (string): The thread ID.
- `status` (string): `in_progress`, `incomplete`, or `completed`.
- `role` (string): `user` or `assistant`.
- `content` (array): Content of the message.
- `assistant_id` (string or null)
- `run_id` (string or null)
- `file_ids` (array): List of file IDs.
- `metadata` (map): Additional information.

## Create Run

### Request

- **Endpoint**: `POST https://api.openai.com/v1/threads/{thread_id}/runs`
- **Path Parameters**:
  - `thread_id` (string, required)
- **Request Body**:
  - `assistant_id` (string, required)
  - Various optional parameters such as `model`, `instructions`, etc.

### Response

Returns a `run` object.

**Example Request**
```bash
curl https://api.openai.com/v1/threads/thread_abc123/runs \
  -H "Authorization: Bearer $OPENAI_API_KEY" \
  -H "Content-Type: application/json" \
  -H "OpenAI-Beta: assistants=v1" \
  -d '{
    "assistant_id": "asst_abc123"
  }'
```

**Example Response**
```json
{
  "id": "run_abc123",
  "object": "thread.run",
  "created_at": 1699063290,
  "assistant_id": "asst_abc123",
  "thread_id": "thread_abc123",
  "status": "queued",
  "started_at": 1699063290,
  "expires_at": null,
  "cancelled_at": null,
  "failed_at": null,
  "completed_at": 1699063291,
  "last_error": null,
  "model": "gpt-4-turbo",
  "tools": [
    {
      "type": "code_interpreter"
    }
  ],
  "file_ids": [
    "file-abc123",
    "file-abc456"
  ],
  "metadata": {},
  "temperature": 1.0,
  "top_p": 1.0,
  "max_prompt_tokens": 1000,
  "max_completion_tokens": 1000,
  "truncation_strategy": {
    "type": "auto",
    "last_messages": null
  },
  "response_format": "auto",
  "tool_choice": "auto"
}
```

## Run Object

Represents an execution run on a thread.

### Fields

- `id` (string): The identifier.
- `object` (string): Object type, always `thread.run`.
- `created_at` (integer): Unix timestamp for creation.
- `thread_id` (string): Thread ID.
- `assistant_id` (string): Assistant ID used for the run.
- `status` (string): Status such as `queued`, `in_progress`.
- `tools` (array): List of tools used.
- `file_ids` (array): List of file IDs.
- `metadata` (map): Additional key-value pairs.
- `temperature` (number): Sampling temperature.
- `max_prompt_tokens` (integer): Max tokens for prompts, and other fields related to the execution settings.

## Use of Streaming in Runs

Streaming is facilitated by passing `"stream": true` in requests to endpoints like `Create Thread and Run`, `Create Run`, and `Submit Tool Outputs`. The responses stream as server-sent events, aiding in real-time interaction.

## Events and Deltas

Several types of events such as `thread.message.delta`, `thread.run.step.delta`, and `message delta object` are emitted during streaming. These convey changes in state or data associated with a message or run step. 

**Event Handling**

Handle unknown events gracefully using mechanisms in the Assistants API to capture streaming events, especially during run executions. 

For further information and details on integrating these streaming functionalities and handling, consult examples and guides for effective implementation in varied environments like Node.js or Python.

# Realtime API Documentation

This document provides a comprehensive guide to implementing a full server/client system that utilizes the Realtime API, with an emphasis on both client and server events. It's crucial for the system to be designed in consideration of the structures and types highlighted below.

## Client Events

These events illustrate how clients can interact in real-time, spanning from session updates to audio buffer manipulation and conversation item management.

### Session Events
- **Session Update**  
  - `update-event_id`: Identifier for the session update event.
  - `update-type`: Type of update applied to the session.
  - `update-session`: Data containing the session update details.

### Input Audio Buffer Events

- **Append Audio**  
  - `append-event_id`: Identifier for the audio append event.
  - `append-type`: Event type for appending audio.
  - `append-audio`: The audio data being appended.

- **Commit Buffer**  
  - `commit-event_id`: Identifier for buffer commit event.
  - `commit-type`: Event type for committing audio input buffer.

- **Clear Buffer**  
  - `clear-event_id`: Identifier for buffer clear event.
  - `clear-type`: Event type for clearing the audio buffer.

### Conversation Item Events

- **Create Item**  
  - `create-event_id`: Identifier for item creation event.
  - `create-type`: Type of the item creation event.
  - `create-previous_item_id`: ID of the previous item in a sequence.
  - `create-item`: The newly created item.

- **Truncate Item**  
  - `truncate-event_id`: Identifier for item truncation event.
  - `truncate-type`: Event type related to item truncation.
  - `truncate-item_id`: ID of the truncated item.
  - `truncate-content_index`: Index within the item content to truncate.
  - `truncate-audio_end_ms`: Millisecond mark for truncating audio.

- **Delete Item**  
  - `delete-event_id`: Identifier for item deletion event.
  - `delete-type`: Event type concerning item deletion.
  - `delete-item_id`: ID of the item to be deleted.

### Response Events

- **Create Response**  
  - `create-event_id`: Identifier for response creation.
  - `create-type`: Event type for creating a response.
  - `create-response`: The response data being created.

- **Cancel Response**  
  - `cancel-event_id`: Identifier for cancelling a response.
  - `cancel-type`: Event type for response cancellation.

## Server Events

Server events detail the responses and actions the server takes in handling sessions, conversations, buffer management, and responses.

### Error Events

- **Error Occurred**  
  - `error-event_id`: Identifier for an error event.
  - `error-type`: Type of error.
  - `error-error`: Details of the error message.

### Session Events

- **Session Created**  
  - `created-event_id`: Identifier of the session creation.
  - `created-type`: Event type for session creation.
  - `created-session`: Details of the created session.

- **Session Updated**  
  - `updated-event_id`: Identifier for a session update.
  - `updated-type`: Update event type.
  - `updated-session`: Details of the session update.

### Conversation Events

- **Conversation Created**  
  - `created-event_id`: Identifier for conversation creation.
  - `created-type`: Event type for conversation creation.
  - `created-conversation`: Details of the created conversation.

- **Item Created**  
  - `created-event_id`: Identifier for created item in conversation.
  - `created-type`: The type of event for item creation.
  - `created-previous_item_id`: ID of preceding item.
  - `created-item`: The item information.

- **Input Audio Transcription**

  - Completed
    - `completed-event_id`: Identifier for completed transcription.
    - `completed-type`: Event type indicating transcription completion.
    - `completed-item_id`: ID of the item associated with transcription.
    - `completed-transcript`: Completed transcript text.

  - Failed
    - `failed-event_id`: Identifier of failed transcription.
    - `failed-type`: Type indicating failed transcription.
    - `failed-error`: Error information related to failure.

### Response Events

- **Response Created**  
  - `created-event_id`: Identifier for response creation.
  - `created-type`: Event type indicating a response was created.
  - `created-response`: Details of the created response.

- **Response Done**  
  - `done-event_id`: Identifier signifying completion.
  - `done-type`: Event type marking completion.

### Rate Limits

- **Rate Limits Updated**  
  - `updated-event_id`: Identifier for rate limiting update.
  - `updated-type`: Event type corresponding to rate limit changes.
  - `updated-rate_limits`: Details of the new rate limits.

## Conclusion

This guide should provide you with the necessary specifications and events for implementing a real-time communication interface using these categorized client and server events. Each event type is designed to handle specific actions or updates within a session or conversation, making it essential to map out interactions for efficient and comprehensive system design.