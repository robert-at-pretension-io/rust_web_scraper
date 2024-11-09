# Twilio Programmable Voice Documentation

## Getting Started

- [No-code quickstart](https://www.twilio.com/docs/voice/quickstart/no-code-voice-studio-quickstart)
- Language-specific quickstarts:
  - [C#/.NET](https://www.twilio.com/docs/voice/quickstart/csharp)
  - [Java](https://www.twilio.com/docs/voice/quickstart/java)
  - [Node.js](https://www.twilio.com/docs/voice/quickstart/node)
  - [PHP](https://www.twilio.com/docs/voice/quickstart/php)
  - [Python](https://www.twilio.com/docs/voice/quickstart/python)
  - [Ruby](https://www.twilio.com/docs/voice/quickstart/ruby)
  - [Go](https://www.twilio.com/docs/voice/quickstart/go)
  - [Android SDK](https://www.twilio.com/docs/voice/sdks/android/get-started)
  - [iOS SDK (Objective-C and Swift)](https://www.twilio.com/docs/voice/sdks/ios/get-started)
  - [JavaScript SDK](https://www.twilio.com/docs/voice/sdks/javascript/get-started)
  - [React Native SDK](https://www.twilio.com/docs/voice/sdks/react-native/getting-started-with-the-voice-react-native-sdk)

## Tutorials

- [Make outbound phone calls](https://www.twilio.com/docs/voice/tutorials/how-to-make-outbound-phone-calls)
- [Respond to incoming phone calls](https://www.twilio.com/docs/voice/tutorials/how-to-respond-to-incoming-phone-calls)
- [Modify calls in progress](https://www.twilio.com/docs/voice/tutorials/how-to-modify-calls-in-progress)
- [Record phone calls](https://www.twilio.com/docs/voice/tutorials/how-to-record-phone-calls)
- [Create conference calls](https://www.twilio.com/docs/voice/tutorials/how-to-create-conference-calls)
- [Gather user input via keypad](https://www.twilio.com/docs/voice/tutorials/how-to-gather-user-input-via-keypad)
- [Retrieve call logs](https://www.twilio.com/docs/voice/tutorials/how-to-retrieve-call-logs)

## Media Streams

### Overview

Media Streams allows access to the raw audio stream of your Programmable Voice calls via WebSockets. Use cases include real-time transcriptions, sentiment analysis, and integrations with AI chat bots.

### Support for Twilio Regions

Available in Ireland (IE1) and Australia (AU1) Regions for both inbound and outbound calls.

### Unidirectional Media Streams

- Only receives a Call's audio stream.
- Use `<Start><Stream>` TwiML instructions or the [Stream resource API](https://www.twilio.com/docs/voice/api/stream-resource) to initiate.
- Not supported for sending DTMF from your server to Twilio.

### Bidirectional Media Streams

- Allows receiving and sending audio to/from Twilio.
- Starts using `<Connect><Stream>` in TwiML.
- Supports DTMF in the inbound direction only.

### Limits

- Unidirectional: Up to four tracks.
- Bidirectional: One Stream per Call.

### Communicate with Twilio's Media Servers

Ensure secure WebSocket connections from Twilio are allowed and validate `X-Twilio-Signature` for authenticity.

## Resources

- [Stream TwiML Reference](https://www.twilio.com/docs/voice/twiml/stream)
- [WebSocket Messages](https://www.twilio.com/docs/voice/media-streams/websocket-messages)
- [Sample GitHub Repos](https://github.com/twilio/media-streams)

## Need Help?

Access support via [Twilio's support team](https://help.twilio.com) or engage with the community on [Twilio's Stack Overflow Collective](https://stackoverflow.com/collectives/twilio).