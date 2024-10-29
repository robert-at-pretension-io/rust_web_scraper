```markdown
# Messaging API Overview

The Programmable Messaging REST API helps you add robust messaging capabilities to your applications.

With the REST API, you can:

- [Send messages](#send-messages)
- [Fetch, list, update, and delete messages](#fetch-list-update-and-delete-messages)
- [Fetch, list, and delete media](#fetch-list-and-delete-media)
- [Manage your short codes](#manage-your-short-codes)
- [Track message feedback](#track-message-feedback)
- [Manage your Messaging Services](#manage-your-messaging-services)
- [Check SMS pricing by country](#check-sms-pricing-by-country)
- [Retrieve a list of deactivated US phone numbers](#retrieve-a-list-of-deactivated-us-phone-numbers)

## Base URLs

Twilio's APIs are served over HTTPS. To ensure data privacy, unencrypted HTTP is not supported.

The following API Resources use `https://api.twilio.com/2010-04-01` as the base URL:

- Message Resource
- MessageFeedback Resource
- Media Resource

API Resources related to [Messaging Services](https://www.twilio.com/docs/messaging/services) use `https://messaging.twilio.com/v1`:

- [Messaging Services Service Resource](https://www.twilio.com/docs/messaging/api/service-resource)
- [Messaging Services PhoneNumber Subresource](https://www.twilio.com/docs/messaging/api/phonenumber-resource)
- [Messaging Services ShortCode Subresource](https://www.twilio.com/docs/messaging/api/services-shortcode-resource)
- [Messaging Services AlphaSender Subresource](https://www.twilio.com/docs/messaging/api/alphasender-resource)
- [Messaging Services ChannelSender Subresource](https://www.twilio.com/docs/messaging/api/messaging-service-channelsender-resource)

The [Deactivations Resource](https://www.twilio.com/docs/messaging/api/deactivations-resource) also uses the `https://messaging.twilio.com/v1` base URL.

[Pricing information for Messaging](https://www.twilio.com/docs/messaging/api/pricing) uses the `https://pricing.twilio.com/v1` base URL.

## Authentication

HTTP requests to the API use [HTTP basic authentication](https://en.wikipedia.org/wiki/Basic_access_authentication).

Use your Twilio **Account SID** as the username and your **Auth Token** (found in [the Console](https://www.twilio.com/console)) as the password for HTTP basic authentication when sending HTTP requests to Twilio.

The code samples in the API documentation show how and where to put your Account SID and Auth Token if you're using a Helper Library or curl to communicate with Twilio's API.

The example below shows how to send your Account SID and Auth Token when using a curl command.

```shell
curl -G https://api.twilio.com/2010-04-01/Accounts \
     -u <YOUR_ACCOUNT_SID>:<YOUR_AUTH_TOKEN>
```

To learn more about interacting with Twilio's REST API, check out the Usage docs on [sending requests to Twilio's REST APIs](https://www.twilio.com/docs/usage/requests-to-twilio), [Twilio's response to API requests](https://www.twilio.com/docs/usage/twilios-response), and [security documentation](https://www.twilio.com/docs/usage/security).

## Use the Programmable Messaging API

Twilio monitors messages to prevent content violating the [Acceptable Use Policy](https://www.twilio.com/en-us/legal/aup) (AUP). This helps to ensure that Twilio Messaging is seen as a trustworthy, high engagement channel and will not slow down the delivery of messages.

If a message you send has violated the AUP, it will be returned and you will receive an error code which identifies the necessary changes you need to make before sending it again.

## Send messages

To send an outbound message, send a `POST` request to the [Message Resource](https://www.twilio.com/docs/messaging/api/message-resource).

- To [send media messages](https://www.twilio.com/docs/messaging/tutorials/how-to-send-sms-messages), use the `MediaUrl` parameter in the request.
- To [schedule an outbound Message](https://www.twilio.com/docs/messaging/features/message-scheduling) to be sent in the future, use the `ScheduleType` and `SendAt` parameters in the request.
- To [send messages with shortened links](https://www.twilio.com/docs/messaging/features/link-shortening), use the `ShortenUrls` parameter in the request.

  **Note:** This feature is available only if you are using [Messaging Services](https://www.twilio.com/docs/messaging/services).

Looking for information on how to receive and reply to messages? [Check out the Receive and Reply to Messages Guide](https://www.twilio.com/docs/messaging/tutorials/how-to-receive-and-reply).

## Fetch, list, update, and delete messages

Use the [Message Resource](https://www.twilio.com/docs/messaging/api/message-resource) to fetch, list, and delete Messages associated with your Account.

Redact messages by updating a Message Resource.

## Fetch, list, and delete media

Twilio creates a [Media Subresource](https://www.twilio.com/docs/messaging/api/media-resource) when an incoming or outgoing Message Resource contains media.

You can fetch, list, and delete Media Subresources.

## Manage your short codes

Fetch, list, and update your Account's short codes with the [ShortCode Resource](https://www.twilio.com/docs/messaging/api/short-code-resource).

## Track message feedback

Track user-reported outcomes of Messages with the [MessageFeedback Subresource](https://www.twilio.com/docs/messaging/api/message-feedback-resource).

## Manage your Messaging Services

Create, fetch, read, update, and delete Messaging Services with the [Service Resource](https://www.twilio.com/docs/messaging/api/service-resource).

Manage your Messaging Services' senders with:

- [The Messaging Services ShortCode Subresource](https://www.twilio.com/docs/messaging/api/services-shortcode-resource)
- [The Messaging Services AlphaSender Subresource](https://www.twilio.com/docs/messaging/api/alphasender-resource)
- [The Messaging Services PhoneNumber Subresource](https://www.twilio.com/docs/messaging/api/phonenumber-resource)
- [The Messaging Services ChannelSender Subresource](https://www.twilio.com/docs/messaging/api/messaging-service-channelsender-resource)

## Check SMS pricing by country

See inbound and outbound SMS message pricing with the [Country Pricing Resource](https://www.twilio.com/docs/messaging/api/pricing).

## Retrieve a list of deactivated US phone numbers

Fetch a list of all US phone numbers that were deactivated on a given day with the [Deactivations Resource](https://www.twilio.com/docs/messaging/api/deactivations-resource).

## Additional Resources

- Send and receive messages in your programming language of choice by using one of Twilio's [Helper Libraries](https://www.twilio.com/docs/libraries). For step-by-step instructions on how to send and receive messages with a Helper Library, choose a quickstart below:
  - [Python](https://www.twilio.com/docs/messaging/quickstart/python)
  - [PHP](https://www.twilio.com/docs/messaging/quickstart/php)
  - [Node.js](https://www.twilio.com/docs/messaging/quickstart/node)
  - [Java](https://www.twilio.com/docs/messaging/quickstart/java)
  - [C#](https://www.twilio.com/docs/messaging/quickstart/csharp-dotnet-framework)
  - [Ruby](https://www.twilio.com/docs/messaging/quickstart/ruby)
- [Check out the Blog for inspiration](https://www.twilio.com/blog/search-results?search=sms) on how you can build Messaging applications with a variety of languages and tools.
- For help troubleshooting your Programmable Messaging application, check out the docs on [Debugging Common Issues](https://www.twilio.com/docs/messaging/guides/debugging-common-issues) and [Debugging Tools](https://www.twilio.com/docs/messaging/guides/debugging-tools).
- [Learn more about Twilio's Global Infrastructure](https://www.twilio.com/docs/global-infrastructure), which allows you to control where your application's Twilio-related data is routed, processed, and stored.
```