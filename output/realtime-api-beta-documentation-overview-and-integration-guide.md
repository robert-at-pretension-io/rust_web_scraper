# Realtime API
## Beta

Build low-latency, multi-modal experiences with Realtime API.

The Realtime API enables you to build low-latency, multi-modal conversational experiences. It currently supports **text and audio** as both **input** and **output**, as well as **[function calling][50]**.

Notable benefits of the API include:

1. **Native speech-to-speech:** Low latency with nuanced output by skipping intermediate text format.
2. **Natural, steerable voices:** Models have natural inflection and can adapt to various tones.
3. **Simultaneous multimodal output:** Text is useful for moderation; faster-than-realtime audio ensures stable playback.

The Realtime API is in beta, and we don't offer client-side authentication at this time. Applications need to route audio from the client to an application server for secure authentication with the Realtime API.

Network conditions can impact audio reliability. For client-side or telephony applications where network reliability is not controlled, using a third-party solution is recommended. Consider our partners' integrations.

## Quickstart

The Realtime API is a server-side WebSocket interface. To help you get started, we have a console demo application showcasing API features.

**Note:** The frontend patterns in the demo app are not recommended for production use. However, they help visualize and inspect Realtime integration.

### Get Started

To start quickly, download and configure the Realtime console demo.

[Get started with the Realtime console][51]

Use partner integrations for frontend applications:

- [LiveKit integration guide][52]
- [Twilio integration guide][53]
- [Agora integration quickstart][54]

## Overview

The Realtime API is a **stateful**, **event-based** API that uses WebSocket. Connection parameters are:

- **URL:** `wss://api.openai.com/v1/realtime`
- **Query Parameters:** `?model=gpt-4o-realtime-preview-2024-10-01`
- **Headers:**
  - `Authorization: Bearer YOUR_API_KEY`
  - `OpenAI-Beta: realtime=v1`

### Example

Using the `ws` library in Node.js:

```javascript
import WebSocket from "ws";

const url = "wss://api.openai.com/v1/realtime?model=gpt-4o-realtime-preview-2024-10-01";
const ws = new WebSocket(url, {
    headers: {
        "Authorization": "Bearer " + process.env.OPENAI_API_KEY,
        "OpenAI-Beta": "realtime=v1",
    },
});

ws.on("open", function open() {
    console.log("Connected to server.");
    ws.send(JSON.stringify({
        type: "response.create",
        response: {
            modalities: ["text"],
            instructions: "Please assist the user.",
        }
    }));
});

ws.on("message", function incoming(message) {
    console.log(JSON.parse(message.toString()));
});
```

Refer to the [API reference][56] for a full list of events. You'll exchange events representing text, audio, function calls, and more.

## Concepts

The Realtime API maintains interaction state during a session.

Clients connect via WebSockets to `wss://api.openai.com/v1/realtime` and exchange JSON-formatted events while the session is active.

### State

Session state includes:

- **Session**
- **Input Audio Buffer**
- **Conversations**, a list of Items
- **Responses**, generating a list of Items

Objects:

- **Session:** Single WebSocket connection for exchanging JSON events (text and audio).
- **Conversation:** List of Items initiated at session start.
- **Items:** `message`, `function_call`, or `function_call_output`.

### Input Audio Buffer

The server's audio buffer holds uncommitted client audio. Append audio using `input_audio_buffer.append`.

## Integration Guide

### Audio Formats

Supported formats:

- raw 16-bit PCM audio at 24kHz, mono, little-endian
- G.711 at 8kHz (u-law and a-law)

Encode audio chunks as base64. Use `pydub` for Python or `audio-decode` for Node.js.

### Instructions

Set `instructions` on session or response to control server output. Example:

> Your knowledge cutoff is 2023-10. You are a helpful, witty, and friendly AI. Speak quickly and engage warmly.

## Events

Nine client events you can send and 28 server events you can listen to. See [API reference][67] for full specifications.

Basic implementation recommendation: Refer to [API reference client source][68].

## Client Events

- [session.update][69]
- [input_audio_buffer.append][70]
- [input_audio_buffer.commit][71]
- [input_audio_buffer.clear][72]
- [conversation.item.create][73]

## Server Events

- [error][78]
- [session.created][79]
- [conversation.created][81]
- [response.created][91]
- [response.done][92]

Complete documentation on integration, client events, server events, and more is available through referenced links.