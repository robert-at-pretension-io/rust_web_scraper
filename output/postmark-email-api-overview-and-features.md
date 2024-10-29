# Grow with us: Join Postmark's new referral partner program and start earning
[Learn more](https://postmarkapp.com/lp/referral-partner-program)

[Postmark](https://postmarkapp.com/) | [Log In](https://account.postmarkapp.com/login)

## Why Postmark?
* [Product](#product)
* [Pricing](https://postmarkapp.com/pricing)
* [Resources](#resources)
* [Help](#help)

---

## Product

### Features
* [Email API](https://postmarkapp.com/email-api)
* [SMTP Service](https://postmarkapp.com/smtp-service)
* [Message Streams](https://postmarkapp.com/message-streams)
* [Transactional Email](https://postmarkapp.com/transactional-email)
* [Email Delivery](https://postmarkapp.com/email-delivery)
* [Email Templates](https://postmarkapp.com/email-templates)
* [Inbound Email](https://postmarkapp.com/inbound-email)
* [Analytics & Retention](https://postmarkapp.com/email-analytics)
* [Integrations](https://postmarkapp.com/integrations)

### Postmark For
* [Agencies](https://postmarkapp.com/for/agencies)
* [Enterprise](https://postmarkapp.com/for/enterprise)
* [Startups](https://postmarkapp.com/for/startups)
* [Bootstrapped Startups](https://postmarkapp.com/for/bootstrapped-startups)
* [Side Projects](https://postmarkapp.com/for/side-projects)

### Postmark vs.
* [SendGrid](https://postmarkapp.com/compare/sendgrid-alternative)
* [Mailgun](https://postmarkapp.com/compare/mailgun-alternative)
* [Amazon SES](https://postmarkapp.com/compare/amazon-ses-alternative)
* [SparkPost](https://postmarkapp.com/compare/sparkpost-alternative)
* [Mandrill](https://postmarkapp.com/compare/mandrill-alternative)

---

## Resources
* [Blog](https://postmarkapp.com/blog)
* [API Documentation](https://postmarkapp.com/developer)
* [Getting Started](https://postmarkapp.com/manual)
* [Email Guides](https://postmarkapp.com/guides)
* [Webinars](https://postmarkapp.com/webinars)
* [Videos](https://postmarkapp.com/videos)
* [Podcast](https://postmarkapp.com/podcast)
* [DMARC Digests](https://dmarcdigests.com)
* [Glossary](https://postmarkapp.com/glossary)

## Help
* [Support Center](https://postmarkapp.com/support)
* [Contact Support](https://postmarkapp.com/contact)
* [Talk to Sales](https://postmarkapp.com/talk-to-sales)
* [Status](https://status.postmarkapp.com/)

[Log in](https://account.postmarkapp.com/login) | [Start free trial](https://account.postmarkapp.com/sign_up)

---

## A powerful email API to make email sending a breeze

Easy integration, reliable delivery, great documentation: integrate with the email API that developers love and start sending in minutes.

[Get started—it’s free](https://account.postmarkapp.com/sign_up) | [API documentation →](https://postmarkapp.com/developer)

### Start sending with Postmark’s email API in minutes

Integrating email into your product doesn’t have to be a pain. With our powerful RESTful email APIs and robust [libraries](https://postmarkapp.com/developer/integration/official-libraries) in pretty much every programming language, integrating email is fast and easy—whether you’re sending transactional or bulk email.

Simpler applications can also use our reliable [SMTP service](https://postmarkapp.com/smtp-service).

---

### Code Examples

#### PHP
```php
// Send an email with the Postmark-PHP library
// Learn more -> https://postmarkapp.com/developer/integration/official-libraries#php

// Install with composer
composer require wildbit/postmark-php;

// Import
use Postmark\PostmarkClient;

// Example request
$client = new PostmarkClient("server token");

$sendResult = $client->sendEmail(
  "sender@example.com",
  "receiver@example.com",
  "Hello from Postmark!",
  "This is just a friendly 'hello' from your friends at Postmark."
);
```

#### Ruby
```ruby
# Send an email with the Postmark Ruby Gem
# Learn more -> https://postmarkapp.com/developer/integration/official-libraries#ruby-gem

# Add the Postmark Ruby Gem to your Gemfile
gem 'postmark'

# Require gem
require 'postmark'

# Create an instance of Postmark::ApiClient
client = Postmark::ApiClient.new('POSTMARK_API_TEST')

# Example request
client.deliver(
  from: 'sender@example.com',
  to: 'receiver@example.com',
  subject: 'Hello from Postmark',
  html_body: '<strong>Hello</strong> dear Postmark user.',
  track_opens: true
)
```

#### Python
```python
# Send an email with the Postmark Python library
# Learn more -> https://postmarkapp.com/send-email/python

# Install the Postmark Python library with pip from the command line:
pip install postmarker

# Import
from postmarker.core import PostmarkClient

# Create an instance of the Postmark client
postmark = PostmarkClient(server_token='POSTMARK-SERVER-API-TOKEN-HERE')

# Send an email
postmark.emails.send(
  From='sender@example.com',
  To='recipient@example.com',
  Subject='Postmark test',
  HtmlBody='HTML body goes here'
)
```

---

## Say goodbye to email deliverability issues

Once you integrate with Postmark, you can count on us to get your emails delivered fast and reliably—so you can go back to building great products.

> “Been a Postmark customer for a very long time. Stability and deliverability is so good I sometimes go months without thinking about it. It just works.”  
> — **Ben Webster**, Cofounder of Insured by Us

---

## Ready to get started?

Join thousands of businesses that already trust their email delivery to Postmark.  
[Start free trial](https://account.postmarkapp.com/sign_up)

## Follow us on Twitter
* [Follow @postmarkapp](https://twitter.com/postmarkapp)

## Still have questions?
* [Contact us](https://postmarkapp.com/contact)

---

### Privacy
* [Privacy Policy](https://postmarkapp.com/privacy-policy)
* [Cookie Policy](https://postmarkapp.com/cookie-policy)
* [Terms of Service](https://postmarkapp.com/terms-of-service)
* [EU Data Protection](https://postmarkapp.com/eu-privacy)

© ActiveCampaign, LLC, 2024.