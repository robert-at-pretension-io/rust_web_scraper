# How to Use Our REST APIs

Twilio's APIs (Application Programming Interfaces) power its platform for communications. Behind these APIs is a software layer connecting and optimizing communications networks around the world to allow your users to call and message anyone, globally.

## Info

Twilio has a wide range of APIs, from SMS to Voice to Wireless. You can find Twilio's API reference documentation throughout our product documentation. You can [browse the various APIs here](https://www.twilio.com/docs/apis), or jump straight to the API reference for [Programmable SMS](https://www.twilio.com/docs/messaging/api) or [Programmable Voice](https://www.twilio.com/docs/voice/api).

## What is a REST API, anyway?

An API (Application Programming Interface) is a set of rules that allows programs to communicate with each other, exposing data and functionality across the Internet in a consistent format.

REST stands for "Representational State Transfer". This architectural pattern describes how distributed systems can expose a consistent interface. When people use the term "REST API", they typically refer to an API accessed using the HTTP protocol at predefined URLs.

### Resources and Methods
These URLs represent various resources, which can be returned as JSON, HTML, audio files, or images. Resources often have methods that can be performed over HTTP, like `GET`, `POST`, `PUT`, and `DELETE`. Generally:
- Use `POST` to create resources.
- Use `PUT` to update resources.

Twilio provides many separate REST APIs for sending text messages, making phone calls, managing accounts, and more. Each product is its own API but interacts similarly, whether directly over HTTP or through [Twilio's helper libraries](https://www.twilio.com/docs/libraries) available for various programming languages.

## Working with Twilio's APIs

### Authenticate with HTTP

Twilio supports [HTTP Basic authentication](https://en.wikipedia.org/wiki/Basic_access_authentication). This protects URLs on your server so that only you and Twilio can access them. 

To authenticate:
```
https://username:password@api.twilio.com/2010-04-01/your_desired_path
```

For HTTP Basic authentication, use your Twilio account SID as your username and your auth token as your password:
```bash
curl -G https://api.twilio.com/2010-04-01/Accounts \
-u '<YOUR_ACCOUNT_SID>:<YOUR_AUTH_TOKEN>'
```

You can find both your account SID and auth token in the [Twilio Console](https://www.twilio.com/console) after [signing up for a free trial](https://www.twilio.com/try-twilio).

If you want to use [API keys](https://www.twilio.com/docs/iam/api-keys/key-resource-v2010) instead, authenticate as follows:
```bash
curl -G https://api.twilio.com/2010-04-01/Accounts \
-u '<YOUR_API_KEY>:<YOUR_API_KEY_SECRET>'
```

### Expected Content Type for API Requests

Twilio's APIs expect the content type of API requests to be either `application/x-www-form-urlencoded` or `multipart/form-data`. While Twilio returns responses in JSON format, the API requests themselves should be formatted as mentioned.

### Authenticate using the Twilio SDKs

Currently, Twilio offers officially supported server-side libraries in the following languages:
- [C#](https://github.com/twilio/twilio-csharp)
- [Java](https://github.com/twilio/twilio-java)
- [Node.js](https://github.com/twilio/twilio-node)
- [PHP](https://github.com/twilio/twilio-php)
- [Python](https://github.com/twilio/twilio-python)
- [Ruby](https://github.com/twilio/twilio-ruby)
- [Go](https://github.com/twilio/twilio-go)

These libraries include a `Utilities` class for request validation, requiring your Account SID and Auth Token found in the [Console](https://www.twilio.com/console).

> **Danger**: Always use environment variables to keep your Account SID and Auth Token secret before sharing any code or deploying to production. Check [our guidance](https://www.twilio.com/blog/how-to-set-environment-variables.html) for setting environment variables.

### How Twilio's APIs use webhooks

[Webhooks](https://www.twilio.com/docs/glossary/what-is-a-webhook) are user-defined HTTP callbacks triggered by events in a web application. Twilio uses webhooks to notify your application when events occur, such as incoming calls or SMS messages. Webhooks are triggered asynchronously.

When the event occurs, Twilio makes an HTTP request (usually `POST` or `GET`) to your configured webhook URL. The request includes details like the body of an incoming message or the incoming phone number, allowing your application to process the event and respond with the necessary instructions.

To handle webhooks with Twilio, you need to build a small web application that can accept HTTP requests. Check our [helper libraries](https://www.twilio.com/docs/libraries) to get started quickly.

## Explore the APIs

### Send an SMS with Twilio's API

Twilio's Programmable SMS API enables sending and receiving [SMS messages](https://www.twilio.com/docs/glossary/what-is-an-sms-short-message-service). Sign up for a [free Twilio account](https://www.twilio.com/try-twilio) to get started.

#### Example: Send a simple SMS using the Programmable SMS API
##### Node.js
```javascript
// Download the helper library from https://www.twilio.com/docs/node/install
const twilio = require("twilio");

// Set up your account SID and auth token
const accountSid = process.env.TWILIO_ACCOUNT_SID;
const authToken = process.env.TWILIO_AUTH_TOKEN;
const client = twilio(accountSid, authToken);

async function createMessage() {
    const message = await client.messages.create({
        body: "This is the ship that made the Kessel Run in fourteen parsecs?",
        from: "+15017122661",
        to: "+15558675310",
    });

    console.log(message.body);
}

createMessage();
```

#### Example Output
```json
{
    "account_sid": "ACXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX",
    "body": "This is the ship that made the Kessel Run in fourteen parsecs?",
    "from": "+15017122661",
    "to": "+15558675310",
    "status": "queued",
    ...
}
```

For a complete step-by-step guide to sending and receiving messages with Twilio, check out our [Quickstarts for Programmable SMS](https://www.twilio.com/docs/messaging/quickstart).

### Explore Twilio's other APIs

Twilio offers various REST APIs to integrate powerful communications into your applications, including:
- [Programmable Voice](https://www.twilio.com/docs/voice)
- [Programmable Video](https://www.twilio.com/docs/video)
- [Super SIM](https://www.twilio.com/docs/iot/supersim)
- [Verify](https://www.twilio.com/docs/verify/api) for two-factor authentication

## Need some help?

If you need assistance, reach out to our [support team](https://help.twilio.com), or leverage the community at Twilio's [Stack Overflow Collective](https://stackoverflow.com/collectives/twilio) or browse the [Twilio tag](https://stackoverflow.com/questions/tagged/twilio) on Stack Overflow.