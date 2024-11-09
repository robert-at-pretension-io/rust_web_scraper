# Media Streams - WebSocket Messages

## Support for Twilio Regions

Media Streams is now available in the Ireland (IE1) and Australia (AU1) Regions.

- For outbound calls, [follow the guide for outbound calls in non-US Regions](https://www.twilio.com/docs/global-infrastructure/create-an-outbound-call-via-rest-api-in-a-non-us-twilio-region).
- For inbound calls, [follow the guide for inbound call processing in non-US Regions](https://www.twilio.com/docs/global-infrastructure/inbound-call-processing).
- For more info on Twilio Regions, [visit the Global Infrastructure docs](https://www.twilio.com/docs/global-infrastructure).

During a [Media Stream](https://www.twilio.com/docs/voice/media-streams), Twilio sends messages to your WebSocket server that provide information about the Stream. For [bidirectional Media Streams](https://www.twilio.com/docs/voice/media-streams#bidirectional-media-streams), your server can also send messages back to Twilio.

This page covers each type of message that your WebSocket server can send and receive when using Media Streams.

## WebSocket Messages from Twilio

Twilio sends the following message types to your WebSocket server during a Stream:

- [Connected](#connected-message)
- [Start](#start-message)
- [Media](#media-message)
- [DTMF](#dtmf-message)
- [Stop](#stop-message)
- [Mark](#mark-message) (bidirectional Streams only)

### Connected Message

Twilio sends the `connected` event once a WebSocket connection is established. This is the first message your WebSocket server receives, and this message describes the protocol to expect in the following messages.

| Property   | Description                                           |
|------------|-------------------------------------------------------|
| `event`    | Describes the type of WebSocket message. In this case, `connected`. |
| `protocol` | Defines the protocol for the WebSocket connection's lifetime.       |
| `version`  | Semantic version of the protocol.                                  |

An example `connected` message is shown below.

```json
{
  "event": "connected",
  "protocol": "Call",
  "version": "1.0.0"
}
```

### Start Message

The `start` message contains metadata about the Stream and is sent immediately after the `connected` message. It is only sent once at the start of the Stream.

| Property           | Description                                                    |
|--------------------|----------------------------------------------------------------|
| `event`            | Describes the type of WebSocket message. In this case, `start`.|
| `sequenceNumber`   | Number used to keep track of message sending order. The first message has a value of `1` and then is incremented for each subsequent message. |
| `start`            | An object containing Stream metadata                           |
| `start.streamSid`  | The unique identifier of the Stream                            |
| `start.accountSid` | The SID of the Account that created the Stream                 |
| `start.callSid`    | The SID of the Call that started the Stream                    |
| `start.tracks`     | An array of strings that indicate which media flows are expected in subsequent messages. Values include `inbound`, `outbound`. |
| `start.customParameters` | An object containing the custom parameters that were set when defining the Stream |
| `start.mediaFormat`| An object containing the format of the payload in the `media` messages.|
| `start.mediaFormat.encoding` | The encoding of the data in the upcoming payload. Value is always `audio/x-mulaw`.|
| `start.mediaFormat.sampleRate`| The sample rate in hertz of the upcoming audio data. Value is always `8000` |
| `start.mediaFormat.channels` | The number of channels in the input audio data. Value is always `1` |
| `streamSid`         | The unique identifier of the Stream                          |

An example `start` message is shown below.

```json
{
  "event": "start",
  "sequenceNumber": "1",
  "start": {
    "accountSid": "ACXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX",
    "streamSid": "MZXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX",
    "callSid": "CAXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX",
    "tracks": ["inbound"],
    "mediaFormat": {
      "encoding": "audio/x-mulaw",
      "sampleRate": 8000,
      "channels": 1
    },
    "customParameters": {
      "FirstName": "Jane",
      "LastName": "Doe",
      "RemoteParty": "Bob"
    }
  },
  "streamSid": "MZXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX"
}
```

### Media Message

This message type encapsulates the raw audio data.

| Property          | Description                                                 |
|-------------------|-------------------------------------------------------------|
| `event`           | Describes the type of WebSocket message. In this case, `"media"`.|
| `sequenceNumber`  | Number used to keep track of message sending order. It increments for each subsequent message. |
| `media`           | An object containing media metadata and payload            |
| `media.track`     | One of `inbound` or `outbound`                             |
| `media.chunk`     | The chunk for the message. The first message will begin with `1` and increment with each subsequent message. |
| `media.timestamp` | Presentation Timestamp in Milliseconds from the start of the stream. |
| `media.payload`   | Raw audio in encoded in base64                             |
| `streamSid`       | The unique identifier of the Stream                        |

An example `outbound` media message is shown below. The `payload` value is abbreviated.

```json
{
  "event": "media",
  "sequenceNumber": "3",
  "media": {
    "track": "outbound",
    "chunk": "1",
    "timestamp": "5",
    "payload": "no+JhoaJjpz..."
  },
  "streamSid": "MZXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX"
}
```

An example `inbound` media message is shown below. The `payload` value is abbreviated.

```json
{
  "event": "media",
  "sequenceNumber": "4",
  "media": {
    "track": "inbound",
    "chunk": "2",
    "timestamp": "5",
    "payload": "no+JhoaJjpzS..."
  },
  "streamSid": "MZXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX"
}
```

### Stop Message

Twilio sends a `stop` message when the Stream has stopped or the call has ended.

| Property          | Description                                                 |
|-------------------|-------------------------------------------------------------|
| `event`           | Describes the type of WebSocket message. In this case, `stop`. |
| `sequenceNumber`  | Number used to keep track of message sending order. It increments for each subsequent message. |
| `stop`            | An object containing Stream metadata                       |
| `stop.accountSid` | The Account identifier that created the Stream             |
| `stop.callSid`    | The Call identifier that started the Stream                |
| `streamSid`       | The unique identifier of the Stream                        |

An example `stop` message is shown below.

```json
{
  "event": "stop",
  "sequenceNumber": "5",
  "stop": {
    "accountSid": "ACXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX",
    "callSid": "CAXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX"
  },
  "streamSid": "MZXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX"
}
```

### DTMF Message

The `dtmf` message is currently only supported in [bidirectional Streams](https://www.twilio.com/docs/voice/media-streams#bidirectional-media-streams).

| Property  | Description                                                |
|-----------|------------------------------------------------------------|
| `event`   | Describes the type of WebSocket message. In this case, `dtmf`.|
| `streamSid` | The unique identifier of the Stream                        |
| `sequenceNumber` | Number used to keep track of message sending order.  |
| `dtmf.track` | The track on which the DTMF key was pressed. Its is always `inbound_track` |
| `dtmf.digit`| The number-key tone detected                             |

An example `dtmf` message is shown below. The `dtmf.digit` value is `1`, indicating that someone pressed the `1` key on their handset.

```json
{
  "event": "dtmf",
  "streamSid": "MZXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX",
  "sequenceNumber": "5",
  "dtmf": {
    "track": "inbound_track",
    "digit": "1"
  }
}
```

### Mark Message

Twilio sends the `mark` event only during [bidirectional Streams](https://www.twilio.com/docs/voice/media-streams#bidirectional-media-streams).

| Property    | Description                                              |
|-------------|----------------------------------------------------------|
| `event`     | Describes the type of WebSocket message. In this case, `"mark"`.|
| `streamSid` | The unique identifier of the Stream                      |
| `sequenceNumber` | Number used to keep track of message sending order.  |
| `mark`      | An object containing the mark metadata                   |
| `mark.name` | A custom value. Twilio sends back the `mark.name` you specify when it receives a `mark` message |

An example `mark` message is shown below.

```json
{
  "event": "mark",
  "sequenceNumber": "4",
  "streamSid": "MZXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX",
  "mark": {
    "name": "my label"
  }
}
```

## Send WebSocket Messages to Twilio

If you initiated a Stream using `<Connect><Stream>`, your Stream is [bidirectional](https://www.twilio.com/docs/voice/media-streams#bidirectional-media-streams). This means you can send WebSocket messages back to Twilio, allowing you to pipe audio back into the Call and control the flow of the Stream.

The messages that your WebSocket server can send back to Twilio are:

- [Media](#send-a-media-message)
- [Mark](#send-a-mark-message)
- [Clear](#send-a-clear-message)

### Send a Media Message

To send media back to Twilio, you must provide a properly formatted `media` message.

| Property    | Description                                                  |
|-------------|--------------------------------------------------------------|
| `event`     | Describes the type of WebSocket message. In this case, `"media"`. |
| `streamSid` | The SID of the Stream that should play the audio             |
| `media`     | An object containing the media payload                       |
| `media.payload` | Raw `mulaw/8000` audio in encoded in base64               |

Below is an example `media` message that your WebSocket server sends back to Twilio. The `media.payload` is abbreviated.

```json
{
  "event": "media",
  "streamSid": "MZXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX",
  "media": {
    "payload": "a3242sa..."
  }
}
```

### Send a Mark Message

Send a `mark` event message after sending a [`media` event message](https://www.twilio.com/docs/voice/media-streams/websocket-messages#send-a-media-message) to be notified when the audio that you have sent has been completed.

| Property    | Description                                           |
|-------------|-------------------------------------------------------|
| `event`     | Describes the type of WebSocket message. In this case `"mark"`. |
| `streamSid` | The SID of the Stream that should receive the mark    |
| `mark`      | An object containing mark metadata and payload        |
| `mark.name` | A name specific to your needs that will assist in recognizing future received `mark` event |

Below is an example `mark` message that your WebSocket server sends to Twilio.

```json
{
  "event": "mark",
  "streamSid": "MZXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX",
  "mark": {
    "name": "my label"
  }
}
```

### Send a Clear Message

Send a `clear` message if you want to interrupt the audio that has been sent in various `media` messages. This empties all buffered audio and causes any `mark` messages to be sent back to your WebSocket server.

| Property    | Description                                            |
|-------------|--------------------------------------------------------|
| `event`     | Describes the type of WebSocket message. In this case, `"clear"`. |
| `streamSid` | The SID of the Stream in which you wish to interrupt the audio. |

Below is an example `clear` message that your WebSocket server sends to Twilio.

```json
{
  "event": "clear",
  "streamSid": "MZXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX"
}
```

## Need Some Help?

We all do sometimes; code is hard. Get help now from our [support team](https://help.twilio.com), or lean on the wisdom of the crowd by visiting Twilio's [Stack Overflow Collective](https://stackoverflow.com/collectives/twilio) or browsing the [Twilio tag](https://stackoverflow.com/questions/tagged/twilio) on Stack Overflow.

[Terms of Service](https://www.twilio.com/en-us/legal/tos) | [Privacy Policy](https://www.twilio.com/en-us/legal/privacy)

Copyright © 2024 Twilio Inc.