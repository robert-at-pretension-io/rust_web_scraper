```markdown
# Documentation - HTML API

**Choose your favorite language**
- [Python](https://app.scrapingbee.com/api/v1/?api_key=YOUR-API-KEY&url=YOUR-URL)
- [cURL](https://app.scrapingbee.com/api/v1/?api_key=YOUR-API-KEY&url=YOUR-URL)
- [NodeJS](https://app.scrapingbee.com/api/v1/?api_key=YOUR-API-KEY&url=YOUR-URL)
- [Java](https://app.scrapingbee.com/api/v1/?api_key=YOUR-API-KEY&url=YOUR-URL)
- [Ruby](https://app.scrapingbee.com/api/v1/?api_key=YOUR-API-KEY&url=YOUR-URL)
- [Php](https://app.scrapingbee.com/api/v1/?api_key=YOUR-API-KEY&url=YOUR-URL)
- [Go](https://app.scrapingbee.com/api/v1/?api_key=YOUR-API-KEY&url=YOUR-URL)

## Overview

Here is the list of the different parameters you can use with ScrapingBee's HTML API.

You can also discover this API using our Postman collection covering every ScrapingBee's features.

| Name                | Type                      | Default Value     | Description                                                                         |
|---------------------|--------------------------|--------------------|-------------------------------------------------------------------------------------|
| `api_key`           | string                   | required            | Your API key  [Get your API Key](https://app.scrapingbee.com/api/v1/?api_key=YOUR-API-KEY) |
| `url`               | string                   | required            | The URL of the page you want to scrape [Learn more](https://app.scrapingbee.com/api/v1/?api_key=YOUR-API-KEY) |
| `ai_query`          | string                   | ""                 | Information you want to extract from the webpage using AI [Learn more](https://app.scrapingbee.com/api/v1/?api_key=YOUR-API-KEY) |
| `ai_selector`       | string                   | ""                 | CSS selector to focus the AI extraction on a specific part of the page [Learn more](https://app.scrapingbee.com/api/v1/?api_key=YOUR-API-KEY) |
| `block_ads`         | boolean                  | false              | Block ads on the page you want to scrape [Learn more](https://app.scrapingbee.com/api/v1/?api_key=YOUR-API-KEY) |
| `block_resources`    | boolean                  | true               | Block images and CSS on the page you want to scrape [Learn more](https://app.scrapingbee.com/api/v1/?api_key=YOUR-API-KEY) |
| `cookies`           | string                   | ""                 | Pass custom cookies to the webpage you want to scrape [Learn more](https://app.scrapingbee.com/api/v1/?api_key=YOUR-API-KEY) |
| `country_code`      | string                   | ""                 | Premium proxy geolocation [Learn more](https://app.scrapingbee.com/api/v1/?api_key=YOUR-API-KEY) |
| `custom_google`     | boolean                  | false              | Set to true to scrape Google [Learn more](https://app.scrapingbee.com/api/v1/?api_key=YOUR-API-KEY) |
| `device`            | string                   | "desktop"          | Control the device the request will be sent from [Learn more](https://app.scrapingbee.com/api/v1/?api_key=YOUR-API-KEY) |
| `extract_rules`     | stringified JSON         | ""                 | Data extraction from CSS selectors [Learn more](https://app.scrapingbee.com/api/v1/?api_key=YOUR-API-KEY) |
| `forward_headers`   | boolean                  | false              | Forward particular headers to the webpage, as well as other headers generated by ScrapingBee [Learn more](https://app.scrapingbee.com/api/v1/?api_key=YOUR-API-KEY) |
| `forward_headers_pure` | boolean               | false              | Forward only particular headers to the webpage, and nothing else [Learn more](https://app.scrapingbee.com/api/v1/?api_key=YOUR-API-KEY) |
| `js_scenario`       | stringified JSON         | {}                 | JavaScript scenario to execute [Learn more](https://app.scrapingbee.com/api/v1/?api_key=YOUR-API-KEY) |
| `json_response`     | bool                     | false              | Wrap response in JSON [Learn more](https://app.scrapingbee.com/api/v1/?api_key=YOUR-API-KEY) |
| `own_proxy`         | string                   | ""                 | Allows you to use ScrapingBee with your own proxy provider [Learn more](https://app.scrapingbee.com/api/v1/?api_key=YOUR-API-KEY) |
| `premium_proxy`     | boolean                  | false              | Use premium proxies to bypass difficult to scrape websites [Learn more](https://app.scrapingbee.com/api/v1/?api_key=YOUR-API-KEY) |
| `render_js`         | boolean                  | True               | Render the JavaScript on the page with a headless browser [Learn more](https://app.scrapingbee.com/api/v1/?api_key=YOUR-API-KEY) |
| `return_page_source` | boolean                 | false              | Return the original HTML before the JavaScript rendering [Learn more](https://app.scrapingbee.com/api/v1/?api_key=YOUR-API-KEY) |
| `screenshot`        | boolean                  | false              | Return a screenshot of the page you want to scrape [Learn more](https://app.scrapingbee.com/api/v1/?api_key=YOUR-API-KEY) |
| `screenshot_full_page` | boolean               | false              | Return a screenshot of the full page you want to scrape [Learn more](https://app.scrapingbee.com/api/v1/?api_key=YOUR-API-KEY) |
| `screenshot_selector` | string                 | ""                 | Return a screenshot of a particular area of the page, targeted by a CSS selector [Learn more](https://app.scrapingbee.com/api/v1/?api_key=YOUR-API-KEY) |
| `session_id`        | integer                  | ""                 | Route multiple API requests through the same IP address [Learn more](https://app.scrapingbee.com/api/v1/?api_key=YOUR-API-KEY) |
| `stealth_proxy`     | boolean                  | false              | Use special stealth proxy pool [Learn more](https://app.scrapingbee.com/api/v1/?api_key=YOUR-API-KEY) |
| `timeout`           | int                      | 140000             | Timeout for your requests [Learn more](https://app.scrapingbee.com/api/v1/?api_key=YOUR-API-KEY) |
| `transparent_status_code` | boolean            | false              | Transparently return the same HTTP code of the page requested [Learn more](https://app.scrapingbee.com/api/v1/?api_key=YOUR-API-KEY) |
| `wait`              | integer                  | 0                  | Additional time in ms for JavaScript to render [Learn more](https://app.scrapingbee.com/api/v1/?api_key=YOUR-API-KEY) |
| `wait_browser`      | string                   | domcontentloaded    | Wait until certain browser conditions are met before returning the response [Learn more](https://app.scrapingbee.com/api/v1/?api_key=YOUR-API-KEY) |
| `wait_for`          | string                   | ""                 | CSS / XPath selector to wait for in the DOM [Learn more](https://app.scrapingbee.com/api/v1/?api_key=YOUR-API-KEY) |
| `window_height`     | int                      | 1080               | Height, in pixel, of the viewport used to render the page you want to scrape [Learn more](https://app.scrapingbee.com/api/v1/?api_key=YOUR-API-KEY) |
| `window_width`      | int                      | 1920               | Width, in pixel, of the viewport used to render the page you want to scrape [Learn more](https://app.scrapingbee.com/api/v1/?api_key=YOUR-API-KEY) |

### Getting Started
ScrapingBee is meant to be the easiest scraping API available on the web.

To scrape a web page, you only need two things:
1. Your API key, available [here](https://app.scrapingbee.com/api/v1/?api_key=YOUR-API-KEY)
2. The **encoded** web page URL ([learn more about URL encoding](https://www.w3schools.com/tags/ref_urlencode.asp))

The following snippet is an example of a simple `GET` API call to scrape the URL defined in the query string variable `YOUR-URL`:

```bash
curl "https://app.scrapingbee.com/api/v1/?api_key=YOUR-API-KEY&url=YOUR-URL"
```

### Example Code
#### Python
```python
# Install the Python ScrapingBee library:
# pip install scrapingbee

from scrapingbee import ScrapingBeeClient

client = ScrapingBeeClient(api_key='YOUR-API-KEY')

response = client.get("YOUR-URL")

print('Response HTTP Status Code: ', response.status_code)
print('Response HTTP Response Body: ', response.content)
```

#### NodeJS
```javascript
// Install the Node ScrapingBee library
// npm install scrapingbee
const scrapingbee = require('scrapingbee');

async function get(url) {
  var client = new scrapingbee.ScrapingBeeClient('YOUR-API-KEY');
  var response = await client.get({
    url: url,
    params: { },
  })
  return response
}

get('YOUR-URL').then(function (response) {
    var decoder = new TextDecoder();
    var text = decoder.decode(response.data);
    console.log(text);
}).catch((e) => console.log('A problem occurs: ' + e.response.data));
```

#### Java
```java
import java.io.IOException;
import org.apache.http.client.fluent.*;

public class SendRequest {
  public static void main(String[] args) {
    sendRequest();
  }

  private static void sendRequest() {
    try {
      Content content = Request.Get("https://app.scrapingbee.com/api/v1/?api_key=YOUR-API-KEY&url=YOUR-URL")
        .execute().returnContent();
      System.out.println(content);
    } catch (IOException e) {
      System.out.println(e);
    }
  }
}
```

#### Ruby
```ruby
require 'net/http'
require 'net/https'

def send_request 
    uri = URI('https://app.scrapingbee.com/api/v1/?api_key=YOUR-API-KEY&url=YOUR-URL')

    http = Net::HTTP.new(uri.host, uri.port)
    http.use_ssl = true
    http.verify_mode = OpenSSL::SSL::VERIFY_PEER

    req =  Net::HTTP::Get.new(uri)

    res = http.request(req)
    puts "Response HTTP Status Code: #{ res.code }"
    puts "Response HTTP Response Body: #{ res.body }"
rescue StandardError => e
    puts "HTTP Request failed (#{ e.message })"
end

send_request()
```

#### PHP
```php
<?php
$ch = curl_init();

curl_setopt($ch, CURLOPT_URL, 'https://app.scrapingbee.com/api/v1/?api_key=YOUR-API-KEY&url=YOUR-URL');
curl_setopt($ch, CURLOPT_CUSTOMREQUEST, 'GET');
curl_setopt($ch, CURLOPT_RETURNTRANSFER, 1);

$response = curl_exec($ch);
if (!$response) {
    die('Error: "' . curl_error($ch) . '" - Code: ' . curl_errno($ch));
}

echo 'HTTP Status Code: ' . curl_getinfo($ch, CURLINFO_HTTP_CODE) . PHP_EOL;
echo 'Response Body: ' . $response . PHP_EOL;

curl_close($ch);
?>
```

#### Go
```go
package main

import (
        "fmt"
        "io/ioutil"
        "net/http"
)

func sendClassic() {
        client := &http.Client{}

        req, err := http.NewRequest("GET", "https://app.scrapingbee.com/api/v1/?api_key=YOUR-API-KEY&url=YOUR-URL", nil)

        if err != nil {
                fmt.Println(err)
        }

        resp, err := client.Do(req)
        if err != nil {
                fmt.Println("Failure: ", err)
        }

        respBody, _ := ioutil.ReadAll(resp.Body)

        fmt.Println("Response Status: ", resp.Status)
        fmt.Println("Response Headers: ", resp.Header)
        fmt.Println("Response Body: ", string(respBody))
}

func main() {
    sendClassic()
}
```

### Response
The API will then respond with the raw HTML content of the target URL:
```html
<html>
  <head>
     ...
  </head>
  <body>
     ...
  </body>
</html>
```

Every URL that failed will be retried as many times as possible for 30 seconds. Be aware of the maximum 30-second timeout when your code calls the API.

Headers and cookies returned by the target website are prefixed with `Spb-` (for **S**cra**P**ing**B**ee).

## API Key
### `api_key`
- **Type**: `string` (default= `""`)
- **Required**: Yes

All requests are authenticated using your private API key.

To get access to your API key, create an account [here](https://app.scrapingbee.com/api/v1/?api_key=YOUR-API-KEY) and confirm your email address.
```


```markdown
}

```php
echo 'HTTP Status Code: ' . curl_getinfo($ch, CURLINFO_HTTP_CODE) . PHP_EOL;
echo 'Response Body: ' . $response . PHP_EOL;

// Close curl resource to free up system resources
curl_close($ch);
```

```go
package main

import (
        "fmt"
        "io/ioutil"
        "net/http"
)

func sendClassic() {
        // Create client
        client := &http.Client{}

        // Create request 
        req, err := http.NewRequest("GET", "https://app.scrapingbee.com/api/v1/?api_key=YOUR-API-KEY&url=YOUR-URL&window_height=500", nil)

        parseFormErr := req.ParseForm()
        if parseFormErr != nil {
                fmt.Println(parseFormErr)
        }

        // Fetch Request
        resp, err := client.Do(req)

        if err != nil {
                fmt.Println("Failure : ", err)
        }

        // Read Response Body
        respBody, _ := ioutil.ReadAll(resp.Body)

        // Display Results
        fmt.Println("response Status : ", resp.Status)
        fmt.Println("response Headers : ", resp.Header)
        fmt.Println("response Body : ", string(respBody))
}

func main() {
    sendClassic()
}
```

## Proxies

### Premium Proxy

- `premium_proxy`: **boolean** (default=`false`)
  
For some hard-to-scrape websites, you may need to use premium or residential proxies. These proxies are rarely blocked, and we recommend trying them when you receive error codes from difficult-to-scrape websites, such as search engines or social networks.

To use premium proxies, add the parameter `premium_proxy=true`.

- Each request with this parameter will count as **25 API credits with JavaScript enabled**. If used without [JavaScript rendering](https://docs.scrapingbee.com/javascript-rendering), it will cost **10 credits**.

**Example Usage:**
```bash
curl "https://app.scrapingbee.com/api/v1/?api_key=YOUR-API-KEY&url=YOUR-URL&premium_proxy=True"
```

#### Python Example

Install the Python ScrapingBee library:
```bash
pip install scrapingbee
```

```python
from scrapingbee import ScrapingBeeClient

client = ScrapingBeeClient(api_key='YOUR-API-KEY')

response = client.get('YOUR-URL',
    params = { 
         'premium_proxy': 'True',
    }
)

print('Response HTTP Status Code: ', response.status_code)
print('Response HTTP Response Body: ', response.content)
```

#### NodeJS Example

Install the Node ScrapingBee library:
```bash
npm install scrapingbee
```

```javascript
const scrapingbee = require('scrapingbee');

async function get(url) {
  var client = new scrapingbee.ScrapingBeeClient('YOUR-API-KEY');
  var response = await client.get({
    url: url,
    params: {  
         'premium_proxy': 'True',
    },
  });
  return response;
}

get('YOUR-URL').then(function (response) {
    var decoder = new TextDecoder();
    var text = decoder.decode(response.data);
    console.log(text);
}).catch((e) => console.log('A problem occurs : ' + e.response.data));
```

#### Java Example

```java
import java.io.IOException;
import org.apache.http.client.fluent.*;

public class SendRequest {
  public static void main(String[] args) {
    sendRequest();
  }

  private static void sendRequest() {
    // Classic (GET)
    try {
      Content content = Request.Get("https://app.scrapingbee.com/api/v1/?api_key=YOUR-API-KEY&url=YOUR-URL&premium_proxy=True")
      .execute().returnContent();

      // Print content
      System.out.println(content);
    } catch (IOException e) {
      System.out.println(e);
    }
  }
}
```

#### Ruby Example

```ruby
require 'net/http'
require 'net/https'

# Classic (GET)
def send_request 
    uri = URI('https://app.scrapingbee.com/api/v1/?api_key=YOUR-API-KEY&url=YOUR-URL&premium_proxy=True')

    # Create client
    http = Net::HTTP.new(uri.host, uri.port)
    http.use_ssl = true
    http.verify_mode = OpenSSL::SSL::VERIFY_PEER

    # Create Request
    req = Net::HTTP::Get.new(uri)

    # Fetch Request
    res = http.request(req)
    puts "Response HTTP Status Code: #{ res.code }"
    puts "Response HTTP Response Body: #{ res.body }"
rescue StandardError => e
    puts "HTTP Request failed (#{ e.message })"
end

send_request()
```

#### PHP Example

```php
<?php

// Get cURL resource
$ch = curl_init();

// Set URL 
curl_setopt($ch, CURLOPT_URL, 'https://app.scrapingbee.com/api/v1/?api_key=YOUR-API-KEY&url=YOUR-URL&premium_proxy=True');

// Set method
curl_setopt($ch, CURLOPT_CUSTOMREQUEST, 'GET');

// Return the transfer as a string
curl_setopt($ch, CURLOPT_RETURNTRANSFER, 1);

// Send the request and save response to $response
$response = curl_exec($ch);

// Stop if fails
if (!$response) {
    die('Error: "' . curl_error($ch) . '" - Code: ' . curl_errno($ch));
}

echo 'HTTP Status Code: ' . curl_getinfo($ch, CURLINFO_HTTP_CODE) . PHP_EOL;
echo 'Response Body: ' . $response . PHP_EOL;

// Close curl resource to free up system resources
curl_close($ch);
?>
```

#### Go Example

```go
package main

import (
        "fmt"
        "io/ioutil"
        "net/http"
)

func sendClassic() {
        // Create client
        client := &http.Client{}

        // Create request 
        req, err := http.NewRequest("GET", "https://app.scrapingbee.com/api/v1/?api_key=YOUR-API-KEY&url=YOUR-URL&premium_proxy=True", nil)

        parseFormErr := req.ParseForm()
        if parseFormErr != nil {
                fmt.Println(parseFormErr)
        }

        // Fetch Request
        resp, err := client.Do(req)

        if err != nil {
                fmt.Println("Failure : ", err)
        }

        // Read Response Body
        respBody, _ := ioutil.ReadAll(resp.Body)

        // Display Results
        fmt.Println("response Status : ", resp.Status)
        fmt.Println("response Headers : ", resp.Header)
        fmt.Println("response Body : ", string(respBody))
}

func main() {
    sendClassic()
}
```

### Geolocation

- `country_code`: **string** (default=`""`)

In addition to premium proxies, you can choose the proxy country from a list of countries by using the parameter `country_code=COUNTRY_CODE`.

To use premium proxies from Germany, set both `premium_proxy=true` and `country_code=de` in your API call.

**Country Codes Table:**

| country_code | Country Name      |
|--------------|-------------------|
| br           | Brazil            |
| in           | India             |
| mx           | Mexico            |
| ...          | ...               |
| ru           | Russia            |
| us           | United States     |
| gb           | United Kingdom    |

**Example Usage:**
```bash
curl "https://app.scrapingbee.com/api/v1/?api_key=YOUR-API-KEY&url=YOUR-URL&country_code=de&premium_proxy=True"
```

#### Python Example
```python
from scrapingbee import ScrapingBeeClient

client = ScrapingBeeClient(api_key='YOUR-API-KEY')

response = client.get('YOUR-URL',
    params = { 
         'country_code': 'de',
         'premium_proxy': 'True',
    }
)

print('Response HTTP Status Code: ', response.status_code)
print('Response HTTP Response Body: ', response.content)
```

#### NodeJS Example
```javascript
const scrapingbee = require('scrapingbee');

async function get(url) {
  var client = new scrapingbee.ScrapingBeeClient('YOUR-API-KEY');
  var response = await client.get({
    url: url,
    params: {  
         'country_code': 'de',
         'premium_proxy': 'True',
    },
  });
  return response;
}

get('YOUR-URL').then(function (response) {
    var decoder = new TextDecoder();
    var text = decoder.decode(response.data);
    console.log(text);
}).catch((e) => console.log('A problem occurs : ' + e.response.data));
```

#### Java Example
```java
import java.io.IOException;
import org.apache.http.client.fluent.*;

public class SendRequest {
  public static void main(String[] args) {
    sendRequest();
  }

  private static void sendRequest() {
    try {
      Content content = Request.Get("https://app.scrapingbee.com/api/v1/?api_key=YOUR-API-KEY&url=YOUR-URL&country_code=de&premium_proxy=True")
      .execute().returnContent();

      System.out.println(content);
    } catch (IOException e) {
      System.out.println(e);
    }
  }
}
```

#### Ruby Example
```ruby
require 'net/http'
require 'net/https'

# Classic (GET)
def send_request 
    uri = URI('https://app.scrapingbee.com/api/v1/?api_key=YOUR-API-KEY&url=YOUR-URL&country_code=de&premium_proxy=True')

    # Create client
    http = Net::HTTP.new(uri.host, uri.port)
    http.use_ssl = true
    http.verify_mode = OpenSSL::SSL::VERIFY_PEER

    # Create Request
    req = Net::HTTP::Get.new(uri)

    # Fetch Request
    res = http.request(req)
    puts "Response HTTP Status Code: #{ res.code }"
    puts "Response HTTP Response Body: #{ res.body }"
rescue StandardError => e
    puts "HTTP Request failed (#{ e.message })"
end

send_request()
```

#### PHP Example
```php
<?php

// Get cURL resource
$ch = curl_init();

// Set URL 
curl_setopt($ch, CURLOPT_URL, 'https://app.scrapingbee.com/api/v1/?api_key=YOUR-API-KEY&url=YOUR-URL&country_code=de&premium_proxy=True');

// Set method
curl_setopt($ch, CURLOPT_CUSTOMREQUEST, 'GET');

// Return the transfer as a string
curl_setopt($ch, CURLOPT_RETURNTRANSFER, 1);

// Send the request and save response to $response
$response = curl_exec($ch);

// Stop if fails
if (!$response) {
    die('Error: "' . curl_error($ch) . '" - Code: ' . curl_errno($ch));
}

echo 'HTTP Status Code: ' . curl_getinfo($ch, CURLINFO_HTTP_CODE) . PHP_EOL;
echo 'Response Body: ' . $response . PHP_EOL;

// Close curl resource to free up system resources
curl_close($ch);
?>
```

#### Go Example
```go
package main

import (
        "fmt"
        "io/ioutil"
        "net/http"
)

func sendClassic() {
        // Create client
        client := &http.Client{}

        // Create request 
        req, err := http.NewRequest("GET", "https://app.scrapingbee.com/api/v1/?api_key=YOUR-API-KEY&url=YOUR-URL&country_code=de&premium_proxy=True", nil)

        parseFormErr := req.ParseForm()
        if parseFormErr != nil {
                fmt.Println(parseFormErr)
        }

        // Fetch Request
        resp, err := client.Do(req)

        if err != nil {
                fmt.Println("Failure : ", err)
        }

        // Read Response Body
        respBody, _ := ioutil.ReadAll(resp.Body)

        // Display Results
        fmt.Println("response Status : ", resp.Status)
        fmt.Println("response Headers : ", resp.Header)
        fmt.Println("response Body : ", string(respBody))
}

func main() {
    sendClassic()
}
```

### Stealth Proxy (Beta)

- `stealth_proxy`: **boolean** (default=`false`)
  
For hard-to-scrape websites, even `premium_proxy=True` may not suffice. We have developed a new pool of proxies that should work for these challenging situations.

To use this option, add `stealth_proxy=True` to your API calls.

**Important Notes:**
- This option currently only operates with [JavaScript rendering](https://docs.scrapingbee.com/javascript-rendering) enabled.
- **Each successful API call using this option will cost 75 credits**, even when using [custom_google](https://docs.scrapingbee.com/custom-google).
  
**Things not supported:** 
- The `infinite_scroll` instruction
- Custom headers and cookies
- The `timeout` parameter
- `xhr` and `evaluate_results` data when using `json_response`

**Example Usage:**
```bash
curl "https://app.scrapingbee.com/api/v1/?api_key=YOUR-API-KEY&url=YOUR-URL&stealth_proxy=True"
```

### Own Proxy

- `own_proxy`: **string** (default=`""`)

If you wish to leverage ScrapingBee's infrastructure with your own proxy, use the `own_proxy` parameter.

The syntax for the proxy information is:
```plaintext
<protocol><username>:<password>@<host>:<port>
```
- Protocol is optional.
- If no port is specified, ScrapingBee will use `1080` (the cURL default).

**Example Usage:**
```bash
curl "https://app.scrapingbee.com/api/v1/?api_key=YOUR-API-KEY&url=YOUR-URL&own_proxy=https%3A%2F%2Fjohndoe%3Apassword%40my_proxy.com%3A1234"
```

### Headers

#### Header Forwarding

- `forward_headers`: **boolean** (default=`false`)

You may need to forward specific headers to the target website. This is done by setting `forward_headers` to `true` and prefixing the headers with **"Spb-"**.

**Example Usage:**
```bash
curl "https://app.scrapingbee.com/api/v1/?api_key=YOUR-API-KEY&url=http%3A%2F%2Fhttpbin.org%2Fheaders%3Fjson&forward_headers=true" \
  -H "Spb-Accept-Language:En-US"
```

### JSON Response

- `json_response`: **bool** (default=`false`)

If you want to integrate ScrapingBee with third-party tools that only accept JSON responses or wish to intercept certain XHR/Ajax requests, you can enable this by setting `json_response=True`.

**Example Usage:**
```bash
curl "https://app.scrapingbee.com/api/v1/?api_key=YOUR-API-KEY&url=YOUR-URL&json_response=True"
```

```python
from scrapingbee import ScrapingBeeClient

client = ScrapingBeeClient(api_key='YOUR-API-KEY')

response = client.get('YOUR-URL',
    params = { 
         'json_response': 'True',
    }
)

print('Response HTTP Status Code: ', response.status_code)
print('Response HTTP Response Body: ', response.content)
```
```javascript
const scrapingbee = require('scrapingbee');

async function get(url) {
  var client = new scrapingbee.ScrapingBeeClient('YOUR-API-KEY');
  var response = await client.get({
    url: url,
    params: {  
         'json_response': 'True',
    },
  });
  return response;
}

get('YOUR-URL').then(function (response) {
    var decoder = new TextDecoder();
    var text = decoder.decode(response.data);
    console.log(text);
}).catch((e) => console.log('A problem occurs : ' + e.response.data));
```

### Return Page Source

To receive the raw HTML from the server before JavaScript execution, use `return_page_source=true`.

```bash
curl "https://app.scrapingbee.com/api/v1/?api_key=YOUR-API-KEY&url=YOUR-URL&return_page_source=True"
```

```python
from scrapingbee import ScrapingBeeClient

client = ScrapingBeeClient(api_key='YOUR-API-KEY')

response = client.get('YOUR-URL',
    params = { 
         'return_page_source': 'True',
    }
)

print('Response HTTP Status Code: ', response.status_code)
print('Response HTTP Response Body: ', response.content)
```
```javascript
const scrapingbee = require('scrapingbee');

async function get(url) {
  var client = new scrapingbee.ScrapingBeeClient('YOUR-API-KEY');
  var response = await client.get({
    url: url,
    params: {  
         'return_page_source': 'True',
    },
  });
  return response;
}

get('YOUR-URL').then(function (response) {
    var decoder = new TextDecoder();
    var text = decoder.decode(response.data);
    console.log(text);
}).catch((e) => console.log('A problem occurs : ' + e.response.data));
```

### HTML to PDF Conversion

Coming soon.

## Proxy Mode

ScrapingBee also offers a proxy front-end to the API. This can simplify integration with third-party tools.
```

# Access

The ScrapingBee API will handle requests just like any standard request, with request costs, return codes, and default parameters remaining the same as a standard no-proxy request.

**Important:** We recommend disabling [Javascript rendering](#javascript-rendering) in proxy mode, as it is enabled by default. Below are the credentials and configurations used to access proxy mode:

- **HTTP Address:** `proxy.scrapingbee.com:8886`
- **HTTPS Address:** `proxy.scrapingbee.com:8887`
- **Socks5 Address:** `socks.scrapingbee.com:8888`
- **Username:** `YOUR-API-KEY`
- **Password:** `PARAMETERS`

**Important:** Replace `PARAMETERS` with the supported API parameters. If uncertain, you can start with `render_js=False`.

### Alternative URL Configuration

You can also use URLs structured like this:

```json
{
  "url_http": "http://YOUR-API-KEY:[email protected]:8886",
  "url_https": "https://YOUR-API-KEY:[email protected]:8887",
  "url_socks5": "socks5://YOUR-API-KEY:[email protected]:8888"
}
```

**Important:** Scraping Google using this mode will cost **20 credits** per request.

For more integration details with different tools and languages, visit our [dedicated page](#).

## Session

### `session_id` [`integer`] (default=`""`)

All API requests that use the same `session_id` will be routed through the same IP address for **5 minutes**. It is advisable to use a random integer between `0` and `10,000,000` each time you wish to generate a new `session_id`.

### Example Usage

- **Python:**

```python
# Install the Python ScrapingBee library
# pip install scrapingbee

from scrapingbee import ScrapingBeeClient

client = ScrapingBeeClient(api_key='YOUR-API-KEY')

response = client.get('YOUR-URL', params={'session_id': '123'})

print('Response HTTP Status Code: ', response.status_code)
print('Response HTTP Response Body: ', response.content)
```

- **cURL:**

```bash
curl "https://app.scrapingbee.com/api/v1/?api_key=YOUR-API-KEY&url=YOUR-URL&session_id=123"
```

- **NodeJS:**

```javascript
const scrapingbee = require('scrapingbee');

async function get(url) {
  var client = new scrapingbee.ScrapingBeeClient('YOUR-API-KEY');
  var response = await client.get({
    url: url,
    params: { 'session_id': '123' },
  });
  return response;
}

get('YOUR-URL').then(function (response) {
  console.log(response.data);
}).catch((e) => console.log('A problem occurs: ' + e.response.data));
```

- **Java:**

```java
import org.apache.http.client.fluent.*;

public class SendRequest {
  public static void main(String[] args) {
    sendRequest();
  }

  private static void sendRequest() {
    try {
      Content content = Request.Get("https://app.scrapingbee.com/api/v1/?api_key=YOUR-API-KEY&url=YOUR-URL&session_id=123")
        .execute().returnContent();
      System.out.println(content);
    } catch (IOException e) { 
      System.out.println(e); 
    }
  }
}
```

(Continue with other languages accordingly...)

## Timeout

### `timeout` [`int`] (default=`140000`)

The maximum number of milliseconds that ScrapingBee will wait before returning results, in the range of `1000` to `140000`. Use `timeout=45000` in your API call to modify it.

**Important:** A 0.5-second margin of error exists between the `timeout` provided in your API call and the actual maximum duration.

### Example Usage

- **Python:**

```python
# pip install scrapingbee

from scrapingbee import ScrapingBeeClient

client = ScrapingBeeClient(api_key='YOUR-API-KEY')

response = client.get('YOUR-URL', params={'timeout': '10000'})

print('Response HTTP Status Code: ', response.status_code)
print('Response HTTP Response Body: ', response.content)
```

(Continue with other languages accordingly...)

## Custom Cookies

### `cookies` [`string`] (default=`""`)

To pass custom cookies to the target webpage, include a cookie string in the `cookies` parameter. Multiple cookies should be separated by `;`.

### Cookie Syntax

```
name=value,other_attributes=other_attributes_value;
```

### Example Usage

- **Python:**

```python
from scrapingbee import ScrapingBeeClient

client = ScrapingBeeClient(api_key='YOUR-API-KEY')
response = client.get('http://httpbin.org/cookies?json', 
                      cookies={"cookie_name_1": "cookie_value1", 
                               "cookie_name_2": "cookie_value_2"})

print('Response HTTP Status Code: ', response.status_code)
print('Response HTTP Response Body: ', response.content)
```

(Continue with other languages accordingly...)

## Device

### `device` [`string`] (default=`"desktop"`)

Choose the type of device that will send the request to the server, either `desktop` (default) or `mobile`.

### Example Usage

- **Python:**

```python
from scrapingbee import ScrapingBeeClient

client = ScrapingBeeClient(api_key='YOUR-API-KEY')

response = client.get('YOUR-URL', params={'device': 'desktop'})

print('Response HTTP Status Code: ', response.status_code)
print('Response HTTP Response Body: ', response.content)
```

(Continue with other languages accordingly...)

## Google

### `custom_google` [`boolean`] (default=`false`)

If scraping Google domains (`google.com`, `news.google.com`, etc.), use `custom_google=True`. Each request using this option costs **20 credits**.

### Example Usage

- **Python:**

```python
from scrapingbee import ScrapingBeeClient

client = ScrapingBeeClient(api_key='YOUR-API-KEY')

response = client.get('https://www.google.com', params={'custom_google': 'True'})

print('Response HTTP Status Code: ', response.status_code)
print('Response HTTP Response Body: ', response.content)
```

(Continue with other languages accordingly...)

## POST / PUT

To send a POST/PUT request, send it to the main endpoint with your `api_key` and `url` parameters. Data will be forwarded transparently.

### Example Usage

- **Python:**

```python
import requests

response = requests.post(
    url="https://app.scrapingbee.com/api/v1",
    params={"url": "https://httpbin.org/anything", "api_key": "YOUR-API-KEY"},
    headers={"Content-Type": "application/x-www-form-urlencoded; charset=utf-8"},
    data={"KEY_1": "VALUE_1"},
)

print('Response HTTP Status Code: ', response.status_code)
print('Response HTTP Response Body: ', response.content)
```

(Continue with other languages accordingly...)

## Credit Cost for Your Requests

| Feature Used                                   | API Credit Cost |
|------------------------------------------------|------------------|
| Rotating Proxy without JavaScript rendering     | 1                |
| Rotating Proxy with JavaScript rendering (default)| 5                |
| Premium Proxy without JavaScript rendering      | 10               |
| Premium Proxy with JavaScript rendering         | 25               |
| Stealth Proxy without JavaScript rendering      | (coming soon)    |
| Stealth Proxy with JavaScript rendering         | 75               |

## Usage Endpoint

To monitor credit consumption and concurrency usage programmatically, use the `/usage` endpoint. This can be called **6** times per minute.

### Example Usage

- **Python:**

```python
import requests

response = requests.get(url="https://app.scrapingbee.com/api/v1/usage", params={"api_key": "YOUR-API-KEY"})

print('Response HTTP Status Code: ', response.status_code)
print('Response HTTP Response Body: ', response.content)
```

(Continue with other languages accordingly...)

## Response Status Code

### Status Codes List

| Code | Bill?  | Meaning                                     | Solution                                         |
|------|--------|---------------------------------------------|-------------------------------------------------|
| 200  | Yes    | Successful Call                             |                                                 |
| 400  | No     | Bad Request                                 | Incorrect parameters or parameters type.        |
| 401  | No     | No More Credit Available                    | Upgrade your plan or contact sales.            |
| 404  | Yes    | Requested URL Not Found                     | Provide a valid URL.                            |
| 410  | Yes    | Requested URL Gone                          | Provide a valid URL.                            |
| 413  | No     | File Too Large                             | Request a smaller file.                         |
| 429  | No     | Too Many Concurrent Requests                | Upgrade your plan or contact sales.            |
| 500  | No     | Misc Error                                  | Please retry and see the message in the response body.|

### Transparent HTTP Status Code

### `transparent_status_code` [`boolean`] (default=`false`)

By default, ScrapingBee returns HTTP 500 for non-200 responses. Set `transparent_status_code=true` to return the original status code.

### Example Usage

- **Python:**

```python
from scrapingbee import ScrapingBeeClient

client = ScrapingBeeClient(api_key='YOUR-API-KEY')

response = client.get('YOUR-URL', params={'transparent_status_code': 'True'})

print('Response HTTP Status Code: ', response.status_code)
print('Response HTTP Response Body: ', response.content)
```

(Continue with other languages accordingly...)

## Response Headers

| Name                | Meaning                                          |
|---------------------|--------------------------------------------------|
| `Spb-cost`          | Request cost in credits.                        |
| `Spb-initial-status-code` | The initial status code from the scraped page. Useful for redirects. |
| `Spb-resolved-url`  | The resolved URL of the scraped page. Useful for redirects. |


## Ready to Get Started?

Get access to 1,000 free API credits, no credit card required! [Try ScrapingBee for Free](#).

---

### Legal

* [Terms of Service](#)
* [Privacy Policy](#)
* [GDPR Compliance](#)
* [Data Processing Agreement](#)
* [Cookie Policy](#)
* [Acceptable Use Policy](#)
* [Legal Notices](#)

### Company

* [Team](#)
* [Company's Journey](#)
* [Blog](#)
* [Rebranding](#)
* [Affiliate Program](#)

### Learning Web Scraping

* [Web Scraping Questions](#)
* [A Guide to Web Scraping without Getting Blocked](#)