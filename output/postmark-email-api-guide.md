# Postmark

[Log In](https://account.postmarkapp.com/login)

## Why Postmark?
* [Product](#)

### Features
- [Email API](https://postmarkapp.com/email-api)
- [SMTP Service](https://postmarkapp.com/smtp-service)
- [Message Streams](https://postmarkapp.com/message-streams)
- [Transactional Email](https://postmarkapp.com/transactional-email)
- [Email Delivery](https://postmarkapp.com/email-delivery)
- [Email Templates](https://postmarkapp.com/email-templates)
- [Inbound Email](https://postmarkapp.com/inbound-email)
- [Analytics & Retention](https://postmarkapp.com/email-analytics)
- [Integrations](https://postmarkapp.com/integrations)

### Postmark For
- [Agencies](https://postmarkapp.com/for/agencies)
- [Enterprise](https://postmarkapp.com/for/enterprise)
- [Startups](https://postmarkapp.com/for/startups)
- [Bootstrapped Startups](https://postmarkapp.com/for/bootstrapped-startups)
- [Side Projects](https://postmarkapp.com/for/side-projects)

### Postmark vs.
- [SendGrid](https://postmarkapp.com/compare/sendgrid-alternative)
- [Mailgun](https://postmarkapp.com/compare/mailgun-alternative)
- [Amazon SES](https://postmarkapp.com/compare/amazon-ses-alternative)
- [SparkPost](https://postmarkapp.com/compare/sparkpost-alternative)
- [Mandrill](https://postmarkapp.com/compare/mandrill-alternative)

## Pricing
[Pricing](https://postmarkapp.com/pricing)

## Resources
- [Blog](https://postmarkapp.com/blog)
- [API Documentation](https://postmarkapp.com/developer)
- [Getting Started](https://postmarkapp.com/manual)
- [Email Guides](https://postmarkapp.com/guides)
- [Email Comic](https://postmarkapp.com/postmark-express)
- [Webinars](https://postmarkapp.com/webinars)
- [Videos](https://postmarkapp.com/videos)
- [Podcast](https://postmarkapp.com/podcast)
- [Labs](https://postmarkapp.com/labs)
- [DMARC Digests](https://dmarcdigests.com)
- [Glossary](https://postmarkapp.com/glossary)

## Help
- [Support Center](https://postmarkapp.com/support)
- [Contact Support](https://postmarkapp.com/contact)
- [Talk to Sales](https://postmarkapp.com/talk-to-sales)
- [Status](https://status.postmarkapp.com/)

[Log in](https://account.postmarkapp.com/login) | [Start free trial](https://account.postmarkapp.com/sign_up)

## Overview
- [Introduction](https://postmarkapp.com/developer)

## Integration
- [Official libraries](https://postmarkapp.com/developer/integration/official-libraries)
- [Community libraries](https://postmarkapp.com/developer/integration/community-libraries)
- [Tools and Integrations](https://postmarkapp.com/developer/integration/other-tools-and-integrations)

## User Guide
- [Sending email with API](https://postmarkapp.com/developer/user-guide/send-email-with-api)
  - [Send a single email](https://postmarkapp.com/developer/user-guide/send-email-with-api/send-a-single-email)
  - [Send batch emails](https://postmarkapp.com/developer/user-guide/send-email-with-api/batch-emails)
- [Sending email with SMTP](https://postmarkapp.com/developer/user-guide/send-email-with-smtp)
- [Processing email](https://postmarkapp.com/developer/user-guide/inbound)
- [Tracking opens](https://postmarkapp.com/developer/user-guide/tracking-opens)
- [Tracking links](https://postmarkapp.com/developer/user-guide/tracking-links)
- [Managing your account](https://postmarkapp.com/developer/user-guide/managing-your-account)
- [Sandbox mode](https://postmarkapp.com/developer/user-guide/sandbox-mode)

## API Reference
- [Overview](https://postmarkapp.com/developer/api/overview)
- [Email](https://postmarkapp.com/developer/api/email-api)
- [Bulk](https://postmarkapp.com/developer/api/bulk-email)
- [Bounce](https://postmarkapp.com/developer/api/bounce-api)
- [Templates](https://postmarkapp.com/developer/api/templates-api)
- [Server](https://postmarkapp.com/developer/api/server-api)
- [Servers](https://postmarkapp.com/developer/api/servers-api)
- [Message Streams](https://postmarkapp.com/developer/api/message-streams-api)
- [Messages](https://postmarkapp.com/developer/api/messages-api)
- [Domains](https://postmarkapp.com/developer/api/domains-api)
- [Sender signatures](https://postmarkapp.com/developer/api/signatures-api)
- [Stats](https://postmarkapp.com/developer/api/stats-api)
- [Triggers: Inbound rules](https://postmarkapp.com/developer/api/inbound-rules-triggers-api)
- [Webhooks](https://postmarkapp.com/developer/api/webhooks-api)
- [Suppressions](https://postmarkapp.com/developer/api/suppressions-api)
- [Data Removal](https://postmarkapp.com/developer/api/data-removals-api)

## Webhooks
- [Webhooks Overview](https://postmarkapp.com/developer/webhooks/webhooks-overview)
- [Bounce webhook](https://postmarkapp.com/developer/webhooks/bounce-webhook)
- [Inbound webhook](https://postmarkapp.com/developer/webhooks/inbound-webhook)
- [Open tracking webhook](https://postmarkapp.com/developer/webhooks/open-tracking-webhook)
- [Delivery webhook](https://postmarkapp.com/developer/webhooks/delivery-webhook)
- [Click webhook](https://postmarkapp.com/developer/webhooks/click-webhook)
- [Spam complaint webhook](https://postmarkapp.com/developer/webhooks/spam-complaint-webhook)
- [Subscription change webhook](https://postmarkapp.com/developer/webhooks/subscription-change-webhook)
- [SMTP API Error](https://postmarkapp.com/developer/webhooks/smtp-api-error)

## Sending Email with API

### Send a Single Email
To send a single email through Postmark, send an HTTP POST request to the `/email` endpoint with a JSON message attached to the body.

**Example Request with Curl:**
```bash
curl "https://api.postmarkapp.com/email" \
  -X POST \
  -H "Accept: application/json" \
  -H "Content-Type: application/json" \
  -H "X-Postmark-Server-Token: server token" \
  -d '{
  "From": "sender@example.com",
  "To": "receiver@example.com",
  "Subject": "Postmark test",
  "TextBody": "Hello dear Postmark user.",
  "HtmlBody": "<html><body><strong>Hello</strong> dear Postmark user.</body></html>",
  "MessageStream": "outbound"
}'
```

### Authentication Headers
To authenticate to the service, use the `X-Postmark-Server-Token` header. Each Postmark server has its own API token allowing exclusive access.

### JSON Message Format
```json
{
  "From": "sender@example.com",
  "To": "receiver@example.com",
  "Cc": "copied@example.com",
  "Bcc": "blank-copied@example.com",
  "Subject": "Test",
  "Tag": "Invitation",
  "HtmlBody": "<b>Hello</b>",
  "TextBody": "Hello",
  "ReplyTo": "reply@example.com",
  "Metadata": {
      "Color": "blue",
      "Client-Id": "12345"
  },
  "Headers": [
    {
      "Name": "CUSTOM-HEADER",
      "Value": "value"
    }
  ],
  "TrackOpens": true,
  "TrackLinks": "HtmlOnly",
  "MessageStream": "outbound"
}
```

### Attachments
Attachments are specified in the `Attachments` array in the JSON message. Some forbidden file types include vbs, exe, bin, bat, etc.

**Limitations:**
- `TextBody` and `HtmlBody` can be up to 5MB each.
- Total message size, including attachments, can be up to 10MB.

#### Example Message with Attachments
```json
{
  "Attachments": [
    {
      "Name": "readme.txt",
      "Content": "dGVzdCBjb250ZW50",
      "ContentType": "text/plain"
    },
    {
      "Name": "report.pdf",
      "Content": "dGVzdCBjb250ZW50",
      "ContentType": "application/octet-stream"
    }
  ]
}
```

### Success Response
If successful, you will receive a JSON message:
```json
{
  "ErrorCode": 0,
  "Message": "OK",
  "MessageID": "b7bc2f4a-e38e-4336-af7d-e6c392c2f817",
  "SubmittedAt": "2010-11-26T12:01:05.1794748-05:00",
  "To": "receiver@example.com"
}
```

### Send Batch Emails
To send batch emails, use the `/email/batch` endpoint, allowing up to 500 messages in one API call.

**Example Request with Curl:**
```bash
curl "https://api.postmarkapp.com/email/batch" \
  -X POST \
  -H "Accept: application/json" \
  -H "Content-Type: application/json" \
  -H "X-Postmark-Server-Token: server token" \
  -d '[{
    "From": "sender@example.com",
    "To": "receiver1@example.com",
    "Subject": "Postmark test #1",
    "TextBody": "Hello dear Postmark user.",
    "HtmlBody": "<html><body><strong>Hello</strong> dear Postmark user.</body></html>",
    "MessageStream": "outbound"
  },
  {
    "From": "sender@example.com",
    "To": "receiver2@example.com",
    "Subject": "Postmark test #2",
    "TextBody": "Hello dear Postmark user.",
    "HtmlBody": "<html><body><strong>Hello</strong> dear Postmark user.</body></html>",
    "MessageStream": "outbound"
  }]'
```

### Response
A JSON array containing each response for the messages sent in your batched call will be received.

---

* For more information, visit [Postmark](https://postmarkapp.com/).