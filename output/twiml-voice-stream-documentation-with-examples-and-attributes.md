# TwiML™️ Voice: `<Stream>`

The `<Stream>` TwiML noun is used with `<Start>` or `<Connect>`. When Twilio executes `<Start><Stream>` or `<Connect><Stream>`, it forks the raw audio stream and streams it to your WebSocket server in near real-time.

- For more about Media Streams, [visit the overview page](https://www.twilio.com/docs/voice/media-streams).

- `<Start><Stream>` creates a [unidirectional Stream](https://www.twilio.com/docs/voice/media-streams#unidirectional-media-streams).
- `<Connect><Stream>` creates a [bidirectional Stream](https://www.twilio.com/docs/voice/media-streams#bidirectional-media-streams).

## Examples

### Start a MediaStream

```javascript
const VoiceResponse = require('twilio').twiml.VoiceResponse;

const response = new VoiceResponse();
const start = response.start();
start.stream({
  name: 'Example Audio Stream',
  url: 'wss://example.com/audiostream',
});
response.say('The stream has started.');

console.log(response.toString());
```

#### Output

```xml
<?xml version="1.0" encoding="UTF-8"?>
<Response>
    <Start>
        <Stream name="Example Audio Stream" url="wss://example.com/audiostream" />
    </Start>
    <Say>The stream has started.</Say>
</Response>
```

### Connect to a bidirectional MediaStream

```javascript
const VoiceResponse = require('twilio').twiml.VoiceResponse;

const response = new VoiceResponse();
const connect = response.connect();
connect.stream({ url: 'wss://example.com/audiostream' });
response.say('This TwiML instruction is unreachable unless the Stream is ended by your WebSocket server.');

console.log(response.toString());
```

#### Output

```xml
<?xml version="1.0" encoding="UTF-8"?>
<Response>
   <Connect>
       <Stream url="wss://example.com/audiostream" />
   </Connect>
   <Say>This TwiML instruction is unreachable unless the Stream is ended by your WebSocket server.</Say>
</Response>
```

## Attributes

`<Stream>` supports the following attributes:

- **url**: A relative or absolute URL. Default value is none. Only `wss` protocol is supported.
- **name**: Optional. Unique name for the Stream. Default value is none.
- **track**: Optional. Possible values: `inbound_track`, `outbound_track`, `both_tracks`. Default is `inbound_track`.
- **statusCallback**: Optional. URL for status callbacks. Default value is none.
- **statusCallbackMethod**: Optional. HTTP methods: `GET` or `POST`. Default is `POST`.

## Custom Parameters

Include additional key-value pairs using the `<Parameter>` TwiML noun.

```javascript
const VoiceResponse = require('twilio').twiml.VoiceResponse;

const response = new VoiceResponse();
const start = response.start();
const stream = start.stream({
    url: 'wss://mystream.ngrok.io/example'
});
stream.parameter({
    name: 'FirstName',
    value: 'Jane'
});
stream.parameter({
    name: 'LastName',
    value: 'Doe'
});

console.log(response.toString());
```

### Output

```xml
<?xml version="1.0" encoding="UTF-8"?>
<Response>
    <Start>
        <Stream url="wss://mystream.ngrok.io/example">
            <Parameter name="FirstName" value="Jane" />
            <Parameter name="LastName" value="Doe" />
        </Stream>
    </Start>
</Response>
```

## Stop a Stream

Use the `<Stop>` TwiML verb to stop a Stream by `name`.

```xml
<Stop>
   <Stream name="my_first_stream" />
</Stop>
```

To stop a unidirectional Stream via API, visit the [Stream resource API reference page](https://www.twilio.com/docs/voice/api/stream-resource).

---

## Need some help?

Get help from [our support team](https://help.twilio.com) or visit Twilio's [Stack Overflow Collective](https://stackoverflow.com/collectives/twilio).

---

**Legal Notices**

- [Terms of service](https://www.twilio.com/en-us/legal/tos)
- [Privacy Policy](https://www.twilio.com/en-us/legal/privacy)

opyright © 2024 Twilio Inc.