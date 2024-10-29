# Postmark

[Log In](https://account.postmarkapp.com/login)

## Why Postmark?

## Product

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

## Start Free Trial

[Start free trial](https://account.postmarkapp.com/sign_up)

Already have an account? [Log in →](https://account.postmarkapp.com/login)

### Overview

- [Introduction](https://postmarkapp.com/developer)

### Integration

- [Official libraries](https://postmarkapp.com/developer/integration/official-libraries)
- [Community libraries](https://postmarkapp.com/developer/integration/community-libraries)
- [Tools and Integrations](https://postmarkapp.com/developer/integration/other-tools-and-integrations)

### User Guide

- [Sending email with API](https://postmarkapp.com/developer/user-guide/send-email-with-api)
- [Sending email with SMTP](https://postmarkapp.com/developer/user-guide/send-email-with-smtp)
- [Processing email](https://postmarkapp.com/developer/user-guide/inbound)
- [Tracking opens](https://postmarkapp.com/developer/user-guide/tracking-opens)
- [Tracking links](https://postmarkapp.com/developer/user-guide/tracking-links)
- [Managing your account](https://postmarkapp.com/developer/user-guide/managing-your-account)
- [Sandbox mode](https://postmarkapp.com/developer/user-guide/sandbox-mode)

### API Reference

- [Overview](https://postmarkapp.com/developer/api/overview)
- [Email API](https://postmarkapp.com/developer/api/email-api)
  - [Send a single email](https://postmarkapp.com/developer/api/email-api#send-a-single-email)
  - [Send batch emails](https://postmarkapp.com/developer/api/email-api#send-batch-emails)
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

### Webhooks

- [Webhooks overview](https://postmarkapp.com/developer/webhooks/webhooks-overview)
- [Bounce webhook](https://postmarkapp.com/developer/webhooks/bounce-webhook)
- [Inbound webhook](https://postmarkapp.com/developer/webhooks/inbound-webhook)
- [Open tracking webhook](https://postmarkapp.com/developer/webhooks/open-tracking-webhook)
- [Delivery webhook](https://postmarkapp.com/developer/webhooks/delivery-webhook)
- [Click webhook](https://postmarkapp.com/developer/webhooks/click-webhook)
- [Spam complaint webhook](https://postmarkapp.com/developer/webhooks/spam-complaint-webhook)
- [Subscription change webhook](https://postmarkapp.com/developer/webhooks/subscription-change-webhook)
- [SMTP API Error](https://postmarkapp.com/developer/webhooks/smtp-api-error)

## Email API

This endpoint is solely responsible for sending emails with Postmark through a specific server. Note that the batch endpoint accepts up to 500 messages per API call and up to 50 MB payload size, including attachments.

### Send a Single Email

**POST** `/email`

#### Request Headers

```
Content-Type: application/json
Accept: application/json
X-Postmark-Server-Token: <server_token>
```

#### Example Request with curl

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

#### Body Format

| Key         | Type  | Description                                                                                   |
|-------------|-------|-----------------------------------------------------------------------------------------------|
| From        | string| Required: The sender email address (must have a registered and confirmed Sender Signature).  |
| To          | string| Required: Recipient email address (max 50, comma-separated).                                 |
| Cc          | string| Cc recipient email address (max 50, comma-separated).                                       |
| Bcc         | string| Bcc recipient email address (max 50, comma-separated).                                      |
| Subject     | string| Email subject                                                                                 |
| Tag         | string| Email tag to categorize outgoing emails (max 1000 characters).                              |
| HtmlBody    | string| Required if no `TextBody` specified (HTML email message).                                   |
| TextBody    | string| Required if no `HtmlBody` specified (Plain text email message).                             |
| ReplyTo     | string| Reply To override email address (multiple addresses can be comma-separated).                 |
| Headers      | array | List of custom headers to include.                                                           |
| TrackOpens  | bool  | Activate open tracking for this email.                                                      |
| TrackLinks  | string| Activate link tracking for this email (options: `None`, `HtmlAndText`, `HtmlOnly`, `TextOnly`). |
| Metadata    | object| Custom metadata key/value pairs.                                                             |
| Attachments | array | List of attachments.                                                                          |
| MessageStream | string| Set message stream ID (defaults to "outbound").                                            |

#### Example Body Format

```json
{
  "From": "sender@example.com",
  "To": "receiver@example.com",
  "Cc": "copied@example.com",
  "Bcc": "blind-copied@example.com",
  "Subject": "Test",
  "Tag": "Invitation",
  "HtmlBody": "<b>Hello</b> <img src=\"cid:image.jpg\"/>",
  "TextBody": "Hello",
  "ReplyTo": "reply@example.com",
  "Headers": [
    {
      "Name": "CUSTOM-HEADER",
      "Value": "value"
    }
  ],
  "TrackOpens": true,
  "TrackLinks": "None",
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
    },
    {
      "Name": "image.jpg",
      "ContentID": "cid:image.jpg",
      "Content": "dGVzdCBjb250ZW50",
      "ContentType": "image/jpeg"
    }
  ],
  "Metadata": {
      "color": "blue",
      "client-id": "12345"
   },
  "MessageStream": "outbound"
}
```

#### Response

| Key        | Type    | Description                          |
|------------|---------|--------------------------------------|
| To         | string  | Recipient email address              |
| SubmittedAt| string  | Timestamp                            |
| MessageID  | string  | ID of message                        |
| ErrorCode  | integer | [API Error Codes](https://postmarkapp.com/developer/api/overview#error-codes) |
| Message    | string  | Response message                     |

#### Example Response

```http
HTTP/1.1 200 OK
Content-Type: application/json

{
        "To": "receiver@example.com",
        "SubmittedAt": "2014-02-17T07:25:01.4178645-05:00",
        "MessageID": "0a129aee-e1cd-480d-b08d-4f48548ff48d",
        "ErrorCode": 0,
        "Message": "OK"
}
```

### Send Batch Emails

**POST** `/email/batch`

#### Request Headers

```
Content-Type: application/json
Accept: application/json
X-Postmark-Server-Token: <server_token>
```

#### Example Request with curl

```bash
curl "https://api.postmarkapp.com/email/batch" \
  -X POST \
  -H "Accept: application/json" \
  -H "Content-Type: application/json" \
  -H "X-Postmark-Server-Token: server token" \
  -d '[
  {
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
  }
]'
```

#### Body Format

Each message in the batch should follow the same format as the single email endpoint.

#### Example Body Format

```json
[
  {
    "From": "sender@example.com",
    "To": "receiver1@example.com",
    "Cc": "copied@example.com",
    "Bcc": "blank-copied@example.com",
    "Subject": "Test",
    "Tag": "Invitation",
    "HtmlBody": "<b>Hello</b> <img src=\"cid:image.jpg\"/>",
    "TextBody": "Hello",
    "ReplyTo": "reply@example.com",
    "Headers": [
      {
        "Name": "CUSTOM-HEADER",
        "Value": "value"
      }
    ],
    "TrackOpens": true,
    "TrackLinks": "None",
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
      },
     {
       "Name": "image.jpg",
       "ContentID": "cid:image.jpg",
       "Content": "dGVzdCBjb250ZW50",
       "ContentType": "image/jpeg"
     }
    ],
    "Metadata": {
      "color": "green",
      "client-id": "12345"
   },
    "MessageStream": "outbound"
  },
  {
    "From": "sender@example.com",
    "To": "receiver2@example.com",
    "Cc": "copied@example.com",
    "Bcc": "blank-copied@example.com",
    "Subject": "Test",
    "Tag": "Invitation",
    "HtmlBody": "<b>Hello</b> <img src=\"cid:image.jpg\"/>",
    "TextBody": "Hello",
    "ReplyTo": "reply@example.com",
    "Headers": [
      {
        "Name": "CUSTOM-HEADER",
        "Value": "value"
      }
    ],
    "TrackOpens": true,
    "TrackLinks": "None",
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
      },
     {
       "Name": "image.jpg",
       "ContentID": "cid:image.jpg",
       "Content": "dGVzdCBjb250ZW50",
       "ContentType": "image/jpeg"
     }
    ],
    "Metadata": {
      "color": "blue",
      "client-id": "54321"
   },
    "MessageStream": "outbound"
  }
]
```

#### Response

The `/batch` endpoint will return a 200-level HTTP status even if individual messages may fail. Users should check the success and error code for each message in the response.

| Key        | Type    | Description                          |
|------------|---------|--------------------------------------|
| To         | string  | Recipient email address              |
| SubmittedAt| string  | Timestamp                            |
| MessageID  | string  | ID of message                        |
| ErrorCode  | integer | [API Error Codes](https://postmarkapp.com/developer/api/overview#error-codes) |
| Message    | string  | Response message                     |

#### Example Response

```http
HTTP/1.1 200 OK
Content-Type: application/json

[
  {
    "ErrorCode": 0,
    "Message": "OK",
    "MessageID": "b7bc2f4a-e38e-4336-af7d-e6c392c2f817",
    "SubmittedAt": "2010-11-26T12:01:05.1794748-05:00",
    "To": "receiver1@example.com"
  },
  {
    "ErrorCode": 406,
    "Message": "You tried to send to a recipient that has been marked as inactive."
  }
]
```

## Product

- [Pricing](https://postmarkapp.com/pricing)
- [Customers](https://postmarkapp.com/customers)
- [Reviews](https://postmarkapp.com/reviews)
- [Dedicated IPs](https://postmarkapp.com/dedicated-ips)
- [iOS App](https://postmarkapp.com/ios)
- [Referral Partner Program](https://postmarkapp.com/lp/referral-partner-program)
- [Latest Updates](https://postmarkapp.com/updates)

## Features

- [Email API](https://postmarkapp.com/email-api)
- [SMTP Service](https://postmarkapp.com/smtp-service)
- [Message Streams](https://postmarkapp.com/message-streams)
- [Transactional Email](https://postmarkapp.com/transactional-email)
- [Email Delivery](https://postmarkapp.com/email-delivery)
- [Templates](https://postmarkapp.com/email-templates)
- [Inbound Email](https://postmarkapp.com/inbound-email)
- [Analytics & Retention](https://postmarkapp.com/email-analytics)
- [Integrations](https://postmarkapp.com/integrations)
- [Webhooks](https://postmarkapp.com/email-webhooks)
- [Security](https://postmarkapp.com/security)
- [Email Experts](https://postmarkapp.com/email-experts)
- [Rebound](https://postmarkapp.com/rebound)

## Postmark For

- [Agencies](https://postmarkapp.com/for/agencies)
- [Startups](https://postmarkapp.com/for/startups)
- [Enterprise](https://postmarkapp.com/for/enterprise)
- [Bootstrapped Startups](https://postmarkapp.com/for/bootstrapped-startups)
- [Side Projects](https://postmarkapp.com/for/side-projects)
- [Developers](https://postmarkapp.com/send-email)

## Postmark vs.

- [SendGrid](https://postmarkapp.com/compare/sendgrid-alternative)
- [SparkPost](https://postmarkapp.com/compare/sparkpost-alternative)
- [Mailgun](https://postmarkapp.com/compare/mailgun-alternative)
- [Amazon SES](https://postmarkapp.com/compare/amazon-ses-alternative)
- [Mandrill](https://postmarkapp.com/compare/mandrill-alternative)

## Resources

- [Blog](https://postmarkapp.com/blog)
- [API Documentation](https://postmarkapp.com/developer)
- [Getting Started](https://postmarkapp.com/manual)
- [Email Guides](https://postmarkapp.com/guides)
- [Email Comic](https://postmarkapp.com/postmark-express)
- [Videos](https://postmarkapp.com/videos)
- [Podcast](https://postmarkapp.com/podcast)
- [DMARC Digests](https://dmarcdigests.com)
- [Webinars](https://postmarkapp.com/webinars)
- [Labs](https://postmarkapp.com/labs)
- [Migration Guides](https://postmarkapp.com/migration-guides)
- [Newsletter](https://postmarkapp.com/newsletter)
- [Glossary](https://postmarkapp.com/glossary)

## Help

- [Support Center](https://postmarkapp.com/support)
- [Contact Support](https://postmarkapp.com/contact)
- [Talk to Sales](https://postmarkapp.com/talk-to-sales)
- [Service Status](https://status.postmarkapp.com)

## Visit ActiveCampaign for:

- [Marketing Automation](https://www.activecampaign.com/marketing-automation?utm_source=postmark&utm_medium=referral&utm_campaign=postmark_web&utm_content=postmark_footer_nav)
- [CRM & Sales Automation](https://www.activecampaign.com/sales-crm?utm_source=postmark&utm_medium=referral&utm_campaign=postmark_web&utm_content=postmark_footer_nav)
- [Landing Pages](https://www.activecampaign.com/omnichannel/landing-pages?utm_source=postmark&utm_medium=referral&utm_campaign=postmark_web&utm_content=postmark_footer_nav)
- [SMS Automation](https://www.activecampaign.com/omnichannel/sms-marketing?utm_source=postmark&utm_medium=referral&utm_campaign=postmark_web&utm_content=postmark_footer_nav)

### Legal

- [Privacy Policy](https://postmarkapp.com/privacy-policy)
- [Cookie Policy](https://postmarkapp.com/cookie-policy)
- [Terms of Service](https://postmarkapp.com/terms-of-service)
- [EU Data Protection](https://postmarkapp.com/eu-privacy)

© ActiveCampaign, LLC, 2024.